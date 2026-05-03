# 12. ミニプロジェクト: 簡易 grep

これまでの章を総動員して、簡易版の `grep`（minigrep）を作る。

## 学習目標

- 実用的な CLI を Rust で書く流れを把握する
- 引数解析（`clap`）、エラー処理（`anyhow`）、ファイル I/O、テストを統合する
- ライブラリ層と CLI 層を分離する設計に慣れる

## 仕様

```bash
minigrep <pattern> <path...>
  -i, --ignore-case        大文字小文字を無視
  -n, --line-number        行番号を出力
  -c, --count              マッチ件数のみ出力
  -v, --invert-match       マッチしない行を出力
  --color <when>           never|auto|always（デフォルト auto）
```

例:

```bash
minigrep TODO src/lib.rs src/main.rs
minigrep -in error logs/*.log
```

## プロジェクト

```bash
cd code
cargo new ch12-minigrep
cd ch12-minigrep
cargo add clap --features derive
cargo add anyhow
cargo add thiserror
cargo add walkdir              # 後で再帰検索を足したくなったら
cargo add owo-colors           # ターミナル色付け
```

`Cargo.toml`（抜粋）:

```toml
[package]
name = "minigrep"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
thiserror = "2"
owo-colors = "4"
```

## ステップ 1: ライブラリ層を作る

`src/lib.rs`:

```rust
use std::io::{BufRead, BufReader, Read};
use thiserror::Error;

/// 検索条件
#[derive(Debug, Clone)]
pub struct Query {
    pub pattern: String,
    pub ignore_case: bool,
    pub invert: bool,
}

/// 1 件のマッチ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    pub line_no: usize,
    pub line: String,
}

#[derive(Debug, Error)]
pub enum GrepError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// `Read` から読み出してマッチ行を返す。所有権を奪わずに `&Query` を取る。
pub fn search<R: Read>(query: &Query, reader: R) -> Result<Vec<Match>, GrepError> {
    let pat = if query.ignore_case {
        query.pattern.to_lowercase()
    } else {
        query.pattern.clone()
    };

    let mut matches = Vec::new();
    let buf = BufReader::new(reader);

    for (i, line_res) in buf.lines().enumerate() {
        let line = line_res?;
        let target = if query.ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };
        let hit = target.contains(&pat);
        let keep = if query.invert { !hit } else { hit };
        if keep {
            matches.push(Match { line_no: i + 1, line });
        }
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn run(pattern: &str, text: &str, ignore_case: bool, invert: bool) -> Vec<Match> {
        let q = Query {
            pattern: pattern.to_string(),
            ignore_case,
            invert,
        };
        search(&q, Cursor::new(text)).unwrap()
    }

    #[test]
    fn case_sensitive() {
        let m = run("Rust", "I love Rust\nrust is fun\nRust!", false, false);
        assert_eq!(m.len(), 2);
        assert_eq!(m[0].line_no, 1);
        assert_eq!(m[1].line_no, 3);
    }

    #[test]
    fn case_insensitive() {
        let m = run("rust", "I love Rust\nrust is fun\nRust!", true, false);
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn invert() {
        let m = run("rust", "rust\nfoo\nbar", false, true);
        assert_eq!(m.len(), 2);
        assert_eq!(m[0].line, "foo");
    }

    #[test]
    fn empty_input() {
        let m = run("rust", "", false, false);
        assert!(m.is_empty());
    }
}
```

ポイント:

- `Read` トレイトを取って汎用化（ファイル / `Cursor` / `Stdin` どれでも渡せる）
- テストでは `Cursor<&str>` を使い、ファイル不要にする
- エラー型は `thiserror` で
- 純粋ロジックを `lib.rs` に集約。CLI は次のステップで

`cargo test` で全部緑になるか確認。

## ステップ 2: CLI 層

`src/main.rs`:

```rust
use anyhow::{Context, Result};
use clap::Parser;
use minigrep::{search, Query};
use owo_colors::OwoColorize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

/// 簡易 grep
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// 検索パターン
    pattern: String,

    /// 検索対象ファイル（複数指定可）
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// 大文字小文字を無視
    #[arg(short = 'i', long)]
    ignore_case: bool,

    /// 行番号を出力
    #[arg(short = 'n', long)]
    line_number: bool,

    /// マッチ件数のみ出力
    #[arg(short = 'c', long)]
    count: bool,

    /// マッチしない行を出力
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// 色付け: never|auto|always
    #[arg(long, default_value = "auto")]
    color: ColorWhen,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum ColorWhen {
    Never,
    Auto,
    Always,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let use_color = match cli.color {
        ColorWhen::Always => true,
        ColorWhen::Never => false,
        ColorWhen::Auto => atty::is(atty::Stream::Stdout),  // 必要なら cargo add atty
    };

    let query = Query {
        pattern: cli.pattern.clone(),
        ignore_case: cli.ignore_case,
        invert: cli.invert_match,
    };

    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let many_files = cli.files.len() > 1;
    let mut total_matched = 0usize;

    for path in &cli.files {
        let file = File::open(path)
            .with_context(|| format!("failed to open {:?}", path))?;
        let matches = search(&query, file)
            .with_context(|| format!("failed to search {:?}", path))?;

        total_matched += matches.len();

        if cli.count {
            if many_files {
                writeln!(out, "{}: {}", path.display(), matches.len())?;
            } else {
                writeln!(out, "{}", matches.len())?;
            }
            continue;
        }

        for m in &matches {
            let prefix_path = if many_files {
                format!("{}: ", path.display())
            } else {
                String::new()
            };
            let prefix_no = if cli.line_number {
                format!("{}:", m.line_no)
            } else {
                String::new()
            };

            let highlighted = if use_color {
                highlight(&m.line, &query.pattern, query.ignore_case)
            } else {
                m.line.clone()
            };

            writeln!(out, "{prefix_path}{prefix_no}{highlighted}")?;
        }
    }

    if total_matched == 0 {
        std::process::exit(1);    // grep 互換: マッチ無しは終了コード 1
    }

    Ok(())
}

fn highlight(line: &str, pattern: &str, ignore_case: bool) -> String {
    let target = if ignore_case { line.to_lowercase() } else { line.to_string() };
    let pat = if ignore_case { pattern.to_lowercase() } else { pattern.to_string() };

    let mut out = String::new();
    let mut start = 0;
    while let Some(pos) = target[start..].find(&pat) {
        let abs = start + pos;
        out.push_str(&line[start..abs]);
        out.push_str(&format!("{}", line[abs..abs + pat.len()].red().bold()));
        start = abs + pat.len();
    }
    out.push_str(&line[start..]);
    out
}
```

`atty` を入れたくなければ、color 判定を簡略化:

```rust
let use_color = matches!(cli.color, ColorWhen::Always);
```

## ステップ 3: 統合テスト（assert_cmd）

```bash
cargo add --dev assert_cmd predicates
```

`tests/cli.rs`:

```rust
use assert_cmd::Command;
use predicates::str::contains;
use std::io::Write;

fn make_file(content: &str) -> tempfile::NamedTempFile {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    f.write_all(content.as_bytes()).unwrap();
    f
}

#[test]
fn finds_pattern() {
    let f = make_file("Hello Rust\nfoo bar\nrustacean");
    Command::cargo_bin("minigrep").unwrap()
        .arg("Rust")
        .arg(f.path())
        .assert()
        .success()
        .stdout(contains("Hello Rust"));
}

#[test]
fn missing_pattern_exits_1() {
    let f = make_file("nothing here");
    Command::cargo_bin("minigrep").unwrap()
        .arg("zzz")
        .arg(f.path())
        .assert()
        .code(1);
}
```

`tempfile` も必要なら `cargo add --dev tempfile`。

## 拡張案（自由課題）

- 正規表現対応: `regex` クレートを追加し、`Query::pattern` を `Regex` に
- ディレクトリの再帰検索: `walkdir`
- `.gitignore` 尊重: `ignore` クレート（`ripgrep` も使う）
- 並列ファイル処理: `rayon` で複数ファイルを並列検索
- 非同期版: `tokio::fs` で書き直し
- カラー出力の細やか化: マッチ部分だけ色付け / プレビュー前後行表示（`-A` `-B` `-C`）

## 学んだことの自己テスト

このプロジェクトには以下が含まれている。手元のコードでそれぞれ示せると良い。

- [ ] 所有権・借用（`&Query` で渡す、`File` を move で渡す）
- [ ] `Result` と `?`、`anyhow::Context`
- [ ] `thiserror` 派生
- [ ] ジェネリクスとトレイト境界（`R: Read`）
- [ ] イテレータ（`buf.lines().enumerate()`）
- [ ] エラー型の `From` 自動変換
- [ ] 単体テスト（`Cursor` で in-memory 入力）
- [ ] 統合テスト（`assert_cmd`）
- [ ] バイナリとライブラリの分離（`lib.rs` + `main.rs`）
- [ ] 外部 crate の取り込み（clap, anyhow, thiserror）
- [ ] 終了コードの制御（`std::process::exit`）

## ここから先

| やりたいこと | 進む先 |
|----|----|
| Web サーバを書く | `axum`, `tower`, `tracing` |
| DB を扱う | `sqlx`（async）、`diesel`（同期） |
| CLI ツールを増やす | `clap` + `dialoguer` + `indicatif`（プログレスバー）|
| 並列高速化 | `rayon` |
| WASM | `wasm-bindgen`, `leptos`, `dioxus` |
| 組み込み | `embassy`, `embedded-hal` |
| 言語仕様の深掘り | The Rust Reference, Rust for Rustaceans (本) |

## おつかれさま

ここまでで Rust の主要な道具は揃いました。次は実プロダクトに少しずつ取り入れて、コンパイラと喧嘩しながら定着させましょう。EM 視点でも、所有権や型による不変条件の表現は「設計を型で語れる」強い武器になります。
