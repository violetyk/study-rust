# 解説: Option<T>

[01-basics.md](../01-basics.md) の整数オーバーフローや演習 1-3 で出てくる `Option<T>` の解説。

> `Option<T>` = 「値が入っているかもしれない箱」。中身を取り出すには「箱を開ける」操作が必要。

## メンタルモデル

```
Some(42)           None
┌─────────┐       ┌─────────┐
│ 中身: 42 │       │ 空っぽ   │
└─────────┘       └─────────┘
```

定義:

```rust
enum Option<T> {
    Some(T),    // 値あり
    None,       // 値なし
}
```

`T` は任意の型。`Option<i32>` なら i32 が入っているかもしれないし、入っていないかもしれない。

## 他言語の何に当たるか

| 言語 | 「値がない」を表すもの |
|---|---|
| PHP | `null` |
| Python | `None` |
| Ruby | `nil` |
| JavaScript | `null` / `undefined` |
| Go | `nil` ポインタ、`(value, ok)` の 2 値返し |
| Java | `null` または `Optional<T>` |
| Rust | `Option<T>` |

ほとんどの言語に「値がない」を表すマーカーがある。違いはチェックを忘れた時の挙動:

| 言語 | チェックを忘れたらどうなる？ |
|---|---|
| PHP / Ruby / Python | 実行時にエラー（NoneType has no attribute ...） |
| JavaScript | undefined が伝播してバグの温床 |
| Go | nil ポインタアクセスでパニック |
| Rust | コンパイル時にエラー（中身を出すには match 等が必須） |

これが Rust の `Option<T>` の核心。「null 忘れ」のバグをコンパイル時に弾くために、わざと「値そのもの」と「箱に入った値」を別の型にしている。

## どこで出会うか

`Option<T>` は標準ライブラリのあちこちで返ってくる。

```rust
// Vec の要素アクセス
let v = vec![1, 2, 3];
let first: Option<&i32> = v.first();        // Some(&1) or None（空の時）
let item: Option<&i32> = v.get(10);          // None（範囲外）

// HashMap のキーアクセス
let map = HashMap::from([("a", 1)]);
let value: Option<&i32> = map.get("a");      // Some(&1)
let missing: Option<&i32> = map.get("b");    // None

// 文字列の検索
let s = "hello";
let idx: Option<usize> = s.find('e');        // Some(1)
let nope: Option<usize> = s.find('z');       // None

// 環境変数
let path: Option<String> = std::env::var("PATH").ok();

// イテレータ
let nums = [1, 2, 3];
let max: Option<&i32> = nums.iter().max();   // Some(&3) or None（空の時）

// 整数オーバーフロー
let n: i32 = 100;
let result: Option<i32> = n.checked_add(1);  // Some(101)
```

「null が返るかも」という場面が、Rust では全部 `Option<T>` になる。

## Result<T, E> との違い

| 型 | 用途 |
|---|---|
| `Option<T>` | 値があるかないか（理由は問わない） |
| `Result<T, E>` | 成功か失敗か（失敗の理由が知りたい） |

```rust
// Option: 「キーがある？ない？」 — 理由は要らない
let value: Option<&i32> = map.get("key");

// Result: 「parse 成功？失敗？理由は何？」 — 失敗理由を知りたい
let n: Result<i32, ParseIntError> = "abc".parse();
```

`HashMap.get` で「ない」のは普通のこと、エラーじゃない。`parse` の失敗は「不正な入力」というエラーがあるから理由を持つ。

`Option` と `Result` の使い方はほぼ同じ。`Option` を覚えれば `Result` も応用が効く（[parse-and-result.md](parse-and-result.md) 参照）。

## 中身を取り出す方法

| 方法 | コード | 失敗時 |
|---|---|---|
| `match` | `match opt { Some(x) => ..., None => ... }` | パターンで分岐 |
| `if let` | `if let Some(x) = opt { ... }` | None なら何もしない |
| `unwrap` | `opt.unwrap()` | panic |
| `expect` | `opt.expect("msg")` | メッセージ付き panic |
| `unwrap_or` | `opt.unwrap_or(0)` | デフォルト値 |
| `unwrap_or_else` | `opt.unwrap_or_else(\|\| compute())` | クロージャでデフォルト |
| `unwrap_or_default` | `opt.unwrap_or_default()` | T のデフォルト値 (`Default` トレイト) |
| `?` 演算子 | `let x = opt?;` | 関数から早期リターン |
| `map` | `opt.map(\|x\| x + 1)` | None ならそのまま None |
| `and_then` | `opt.and_then(\|x\| further_op(x))` | チェーン処理 |

これらは `Result` でも同じ感覚で使える。

## 同じことの書き方バリエーション

`checked_add` で書き方を比較:

```rust
let n: i32 = 5;

// match（教科書的）
match n.checked_add(1) {
    Some(result) => println!("{result}"),
    None => println!("overflow"),
}

// if let（None 時に何もしないなら）
if let Some(result) = n.checked_add(1) {
    println!("{result}");
}

// map + unwrap_or_else（関数型風）
let msg = n.checked_add(1)
    .map(|r| r.to_string())
    .unwrap_or_else(|| "overflow".to_string());
println!("{msg}");
```

学習中は `match` で OK。慣れてくると `if let` や `map` を使い分けるようになる。

## なぜ難しく感じるか

- 値そのものではなく「箱に入った値」を扱う発想に慣れていない
- ジェネリクス `<T>` の見た目に圧倒される（[05-traits-generics.md](../05-traits-generics.md) でジェネリクスを正面から扱う）
- enum + パターンマッチがセットで来る（[03-structs-enums.md](../03-structs-enums.md)）
- 他言語の null と違って「中身を取り出す手続き」が強制される

これは ch02（所有権）→ ch03（enum と match）→ ch04（Result）と進むと、繰り返し出てきて自然に馴染む。今すぐ完璧に分かる必要はない。

## まとめ

- `Option<T>` = 値が入っているかもしれない箱（`Some(T)` か `None`）
- 他言語の `null` / `nil` / `None` の Rust 版
- 「null 忘れ」のバグをコンパイル時に弾くための仕組み
- 中身を取り出すには `match` / `if let` / `unwrap` などが必要
- `Result<T, E>` の兄弟（理由を持つかどうかの違い）
- 標準ライブラリのあちこちで返ってくるので、慣れていく
