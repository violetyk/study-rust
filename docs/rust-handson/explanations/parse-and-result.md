# 解説: parse / Result / unwrap

[01-basics.md](../01-basics.md) の型変換セクションに出てくる:

```rust
let n: i32 = "42".parse().unwrap();   // 失敗するかも → Result が返る
```

この 1 行に 3 つの概念が詰まっているので分解する。

> 失敗するかもしれない処理は `Result` を返す。`unwrap` は中身を強制取り出し（失敗時は panic）。

## 行を分解

```rust
let n: i32 = "42".parse().unwrap();
//   ───   ─────  ─────   ──────
//    ①     ②      ③       ④
```

| 番号 | 何をしている |
|---|---|
| ① | `n` の型を `i32` と宣言（これが parse へのヒントになる） |
| ② | `&str` の `"42"` |
| ③ | `parse()` 呼び出し → `Result<i32, ParseIntError>` を返す |
| ④ | `unwrap()` で `Result` から成功値の `i32` を取り出す |

## ② → ③: parse は何をしているか

`parse()` は「文字列を任意の型に変換する」汎用メソッド。

```rust
fn parse<F: FromStr>(&self) -> Result<F, F::Err>
```

`F` は呼び出し元が指定する任意の型（`i32`, `f64`, `bool`, ユーザー定義型 ...）。それぞれの型が「文字列からの変換ルール」を `FromStr` トレイトで定義していて、`parse` はそれを呼び出す。

- `"42".parse::<i32>()` → `Result<i32, _>`
- `"3.14".parse::<f64>()` → `Result<f64, _>`
- `"true".parse::<bool>()` → `Result<bool, _>`

「文字列を i32 に変換」と「文字列を f64 に変換」では実装が違うので、どの型に変換するかを Rust に教える必要がある。

## ① が必要な理由

`parse()` は型パラメータ `F` を必要とする。コンパイラはどこかで `F` を知る必要がある。

```rust
let n = "42".parse();           // ❌ type cannot be inferred
let n: i32 = "42".parse();      // ✅ 左辺の型から F = i32 と推論される
let n = "42".parse::<i32>();    // ✅ turbofish で明示
```

3 つ目の `::<i32>` 記法は「turbofish」と呼ばれる。`<>` が魚の口に見えるからこの名前。型注釈の代わりに直接 parse に教える書き方。

## ③: なぜ Result が返るのか

`parse` は失敗する可能性がある。

```rust
"42".parse::<i32>()    // 成功
"abc".parse::<i32>()   // 失敗（数字じゃない）
"99999999999999999".parse::<i32>()   // 失敗（i32 の範囲外）
```

成功か失敗かを表現するために `Result<T, E>` という enum を返す。

```rust
enum Result<T, E> {
    Ok(T),    // 成功: T 型の値が入っている
    Err(E),   // 失敗: E 型のエラーが入っている
}
```

`Result<i32, ParseIntError>` は「成功なら i32、失敗なら ParseIntError」という意味。

## 他言語との対応

| 言語 | エラーの表現 |
|---|---|
| PHP / Java / Python | 例外 (`try-catch`) |
| Go | `(value, error)` の 2 値返し |
| Ruby | 例外 + 一部メソッドで nil 返し |
| Rust | `Result<T, E>` を返す（型に組み込み） |

特に Go の感覚に近い。

```go
n, err := strconv.Atoi("42")
if err != nil {
    // エラー処理
}
```

```rust
let result = "42".parse::<i32>();
match result {
    Ok(n) => { /* 成功 */ }
    Err(e) => { /* エラー処理 */ }
}
```

「エラーは値として返す」思想は同じ。Rust ではそれを 1 つの enum にまとめている、という違い。

## ④: unwrap は何をしているか

`Result<T, E>` から `T` を取り出すメソッド。

```rust
fn unwrap(self) -> T {
    match self {
        Ok(value) => value,
        Err(_) => panic!("called `unwrap()` on an `Err` value"),
    }
}
```

つまり:

- `Ok(42)` だったら `42` を返す
- `Err(...)` だったらプログラムをクラッシュさせる（panic）

```rust
"42".parse::<i32>().unwrap()    // 42
"abc".parse::<i32>().unwrap()   // 実行時に panic で停止
```

学習中・プロトタイプ・「絶対失敗しない」と分かっている時に使う、お手軽だが乱暴な方法。本番コードでは基本使わない。

## より行儀の良い書き方

実務では unwrap の代わりに以下を使い分ける。

### `?` 演算子（エラーを呼び出し元に委ねる）

```rust
fn parse_age(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse()?;   // 失敗したら関数からそのまま Err を返す
    Ok(n)
}
```

### `unwrap_or` / `unwrap_or_else`（デフォルト値）

```rust
let n: i32 = "abc".parse().unwrap_or(0);   // 失敗時は 0 になる
```

これは演習 1-3 の `unwrap_or` で出てくる。

### `match` で明示的にハンドル

```rust
match "42".parse::<i32>() {
    Ok(n) => println!("Got {n}"),
    Err(e) => eprintln!("Parse error: {e}"),
}
```

### `expect`（unwrap + メッセージ）

```rust
let n: i32 = "42".parse().expect("数値であるべき");
// 失敗したらメッセージ付きで panic
```

`expect` は「ここで失敗したらコードのバグ」と分かっている場面で `unwrap` の代わりに使う。失敗時のメッセージで原因が掴みやすくなる。

## どれを使うか

| 場面 | 推奨 |
|---|---|
| 学習中・プロトタイプ | `unwrap` |
| 失敗したら本当にバグ | `expect("バグの説明")` |
| 失敗時にデフォルト値で続けたい | `unwrap_or(default)` |
| エラーを呼び出し元に伝える | `?` 演算子 |
| 成功と失敗で別の処理 | `match` |

実務では `unwrap` を見たら警戒、`?` を使うのが基本。

## 整理

```rust
let n: i32 = "42".parse().unwrap();
//                ───────  ──────
//                ↓        ↓
//                Result<i32, ParseIntError>
//                          を取り出して i32 にする
//                          失敗したら panic
```

3 つのことが起きている:

1. parse の型パラメータが、`n: i32` のヒントから `i32` に決定
2. parse が `Result<i32, ParseIntError>` を返す
3. unwrap が成功値（`i32`）を取り出す。失敗時は panic

## 詳しくは ch04 で

`Result<T, E>` と `?` 演算子は [04-error-handling.md](../04-error-handling.md) で本格的に扱う。今は「parse は失敗するかもしれないから Result が返る、unwrap でとりあえず取り出す」とだけ理解しておけば 01 の演習は進められる。

演習 1-3 で `unwrap_or(0)` が出てくるが、これが「失敗時のデフォルト値」を指定する穏当な版。
