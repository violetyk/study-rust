# 09. テストとドキュメント

Rust はテストとドキュメントが言語仕様レベルで組み込まれている。「テストファースト文化」が標準ライブラリ・主要 crate に根付いている。

## 学習目標

- 単体テスト・統合テストを書ける
- `assert!` 系マクロを使い分けられる
- doctest（ドキュメント中のサンプルコードもテスト）を書ける
- テストカバレッジ・ベンチマークの基本を知る

## プロジェクト

```bash
cd code
cargo new ch09-testing --lib       # ライブラリ crate として作る
cd ch09-testing
```

`--lib` を付けると `src/lib.rs` が作られる。テスト対象を関数として持つのに都合が良い。

## 単体テスト: `#[test]` 属性

`src/lib.rs`:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;     // 親モジュールのものを取り込む

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn add_negative() {
        assert_eq!(add(-1, -1), -2);
    }
}
```

`cargo test` で実行:

```
running 2 tests
test tests::add_works ... ok
test tests::add_negative ... ok

test result: ok. 2 passed; 0 failed
```

ポイント:

- `#[cfg(test)]` でテスト時のみコンパイル（リリースバイナリには含まれない）
- `mod tests` の中に書くのが慣習
- `use super::*` で親モジュールの型・関数を取り込む

## 主要なアサーション

```rust
assert!(condition);                   // bool
assert_eq!(actual, expected);         // ==
assert_ne!(actual, expected);         // !=
assert!(condition, "msg: {}", x);     // メッセージ付き
```

失敗時には差分が綺麗に出る。`Debug` を実装した型なら自動で表示される。

### Result を返すテスト

```rust
#[test]
fn parse_ok() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "42".parse()?;
    assert_eq!(n, 42);
    Ok(())
}
```

`?` が使えるので「途中でエラーが起きたらテスト失敗」を簡潔に書ける。

### should_panic

```rust
#[test]
#[should_panic]
fn divide_by_zero_panics() {
    divide(10, 0);
}

#[test]
#[should_panic(expected = "divide by zero")]
fn divide_by_zero_message() {
    divide(10, 0);
}
```

### 一時的に無効化

```rust
#[test]
#[ignore]
fn slow_test() { ... }
```

`cargo test -- --ignored` で `#[ignore]` だけ実行。

## 実行制御

```bash
cargo test                       # 全部
cargo test add_works             # 名前で絞り込み
cargo test --lib                 # ライブラリだけ
cargo test --test integration    # 統合テストのファイル指定
cargo test -- --test-threads=1   # 順次実行（デフォルトは並列）
cargo test -- --nocapture        # 標準出力を抑制しない
cargo test -- --show-output      # 各テストの出力をまとめて表示
```

## 統合テスト: `tests/` ディレクトリ

外部から見たクレートの振る舞いをテストする。`tests/` 配下のファイルは独立してコンパイルされる。

```
ch09-testing/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

`tests/integration_test.rs`:

```rust
use ch09_testing::add;

#[test]
fn add_via_external() {
    assert_eq!(add(2, 3), 5);
}
```

統合テストでは `use crate_name::...` で公開 API のみ触れる。private には触れない（黒箱テスト）。

## テストの構造化

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod add_tests {
        use super::*;
        #[test] fn positive() { assert_eq!(add(1, 2), 3); }
        #[test] fn negative() { assert_eq!(add(-1, -2), -3); }
    }

    mod sub_tests {
        use super::*;
        #[test] fn positive() { assert_eq!(sub(5, 2), 3); }
    }
}
```

サブモジュールで論理的なグルーピングが可能。

## doctest（ドキュメント中の例もテストされる）

```rust
/// 二つの整数を足して返す。
///
/// # Examples
///
/// ```
/// use ch09_testing::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`cargo test` を実行すると、ドキュメントコメント内のコード例も実行される。これはサンプルが古びないための強力な仕組み。

注意: doctest は `pub` な API に対してしか走らない。

`cargo doc --open` でブラウザでドキュメントを開ける。

## ドキュメントコメント

| 記法 | 用途 |
|-----|-----|
| `///` | アイテム（関数・型・mod）に対するドキュメント |
| `//!` | 自分が含まれるモジュール・crate に対するドキュメント |
| `//` | 通常コメント（ドキュメントにならない） |

```rust
//! このクレートは ... を提供する（lib.rs の先頭に書く）

/// この関数は ... を計算する
///
/// # Examples
///
/// ```
/// // 例
/// ```
///
/// # Panics
///
/// `n == 0` の場合に panic する。
///
/// # Errors
///
/// パースに失敗すると `Err` を返す。
///
/// # Safety
///
/// この関数は `unsafe` であり、呼び出し側が ... を保証する必要がある。
pub fn foo() {}
```

主要な見出し:

- `# Examples` — サンプルコード（doctest）
- `# Panics` — どんな時 panic するか
- `# Errors` — どんな時 Err になるか
- `# Safety` — `unsafe` 関数の前提
- `# Performance` — 計算量・コスト

## モックとフィクスチャ

Rust にはビルトインのモック機構はない。代表的な選択肢:

- `mockall` クレート: trait をモック化
- 普通に依存を trait で受け取って、テスト時は手書きの実装を渡す（Manual mock）

DDD/Clean Arch の「依存性逆転」と素直に相性が良い。

```rust
trait UserRepo {
    fn find(&self, id: u64) -> Option<String>;
}

struct InMemoryRepo(std::collections::HashMap<u64, String>);

impl UserRepo for InMemoryRepo {
    fn find(&self, id: u64) -> Option<String> {
        self.0.get(&id).cloned()
    }
}

fn greet(repo: &dyn UserRepo, id: u64) -> String {
    match repo.find(id) {
        Some(n) => format!("hi, {n}"),
        None => "stranger".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn greet_known_user() {
        let mut m = HashMap::new();
        m.insert(1, "Yuhei".to_string());
        let repo = InMemoryRepo(m);
        assert_eq!(greet(&repo, 1), "hi, Yuhei");
    }
}
```

## ベンチマーク・カバレッジ・スナップショット

| 用途 | ツール |
|-----|------|
| ベンチマーク | `criterion` クレート |
| カバレッジ | `cargo-tarpaulin`, `cargo-llvm-cov` |
| スナップショット | `insta` クレート |
| プロパティテスト | `proptest`, `quickcheck` |
| CLI テスト | `assert_cmd`, `predicates` |

これらは外部 crate で、必要に応じて導入する。

## 演習

📝 **演習 9-1**: 04 章の `parse_age` 関数を `src/lib.rs` に書き、3 つ以上のテストケースを書け（成功 / 失敗 / 境界値）。

📝 **演習 9-2**: 演習 9-1 の関数に doctest を付けよ。`cargo test` で実行されることを確認する。

📝 **演習 9-3**: `tests/cli.rs` を作り、`assert_cmd` で「コマンドラインから実行したときの動作」をテストする例を書け（ツールが必要なら `cargo add --dev assert_cmd`）。

```rust
use assert_cmd::Command;

#[test]
fn runs_with_arg() {
    Command::cargo_bin("ch09-testing")
        .unwrap()
        .arg("42")
        .assert()
        .success();
}
```

## チェックリスト

- [ ] `#[test]` で単体テストを書ける
- [ ] `assert_eq!` `assert!` `should_panic` を使い分けられる
- [ ] 統合テスト `tests/` の役割が言える
- [ ] doctest を書ける
- [ ] `///` と `//!` の違いがわかる

## 落とし穴

⚠️ **テストはデフォルト並列実行**: 共有状態（ファイル書き換え、グローバル変数）を触るテストは衝突する。`#[serial]`（serial_test crate）か `--test-threads=1` で対処。

⚠️ **`println!` がテストでは出ない**: `cargo test -- --nocapture` で見る、もしくは `--show-output`。

⚠️ **doctest のコンパイルは別**: doctest はそれぞれ独立した crate としてコンパイルされる。実行時間が増える。本当に重要な例だけに絞るのが良い。

⚠️ **cfg(test) はテストモードでだけ on**: 統合テストファイル内ではこれを書く必要はない（既にテスト扱い）。

⚠️ **`unwrap()` をテストでは積極的に使ってよい**: 失敗 = テスト失敗、なので OK。本番コードと同じ慎重さは不要。
