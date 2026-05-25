# 解説: 文字列引数は `&str` を取るのが定石

[02-ownership.md](../02-ownership.md) の「関数引数: `&str` を取るのが定石」の補足。

> `&String` は持ち主を `String` に限定する。`&str` は「読む権利」だけ借りるから、呼び出し側の自由度が違う

## なぜ「定石」と言われるか

`&String` を引数にすると、呼び出し側は `String` を持っていないと呼べない。

```rust
fn first_word(s: &String) -> &str { /* ... */ }

let owned = String::from("hello");
first_word(&owned);       // ✅ OK
first_word("hello");      // ❌ expected `&String`, found `&str`
                          //    文字列リテラルが渡せない
```

`&str` で受けると、`String` でもリテラルでも `Box<str>` でも `Cow<str>` でも、いろんな型から渡せる。

```rust
fn first_word(s: &str) -> &str { /* ... */ }

let owned = String::from("hello");
first_word(&owned);       // ✅ OK（&String → &str に自動 deref）
first_word("hello");      // ✅ OK（リテラルは元から &'static str）
```

呼び出し側の選択肢が広がる ―― これが「定石」の正体。

## 仕組み: Deref coercion（参照強制変換）

`&String` を `&str` を期待する場所に渡すと、コンパイラが自動で型変換してくれる。これは `String` が `Deref<Target = str>` を実装しているため。

```
&String  ─[Deref coercion]→  &str
&Vec<T>  ─[Deref coercion]→  &[T]
&Box<T>  ─[Deref coercion]→  &T
&Rc<T>   ─[Deref coercion]→  &T
&Arc<T>  ─[Deref coercion]→  &T
```

Deref coercion は「メソッド呼び出し時の自動 deref」（[reference-and-deref.md](reference-and-deref.md) 参照）と同じ仕組みの「関数引数版」。`Deref` トレイトを実装している型は、その `Target` への参照を期待する場所に自動で変換される。

```rust
// String の Deref 実装（標準ライブラリ）
impl Deref for String {
    type Target = str;
    fn deref(&self) -> &str { /* ヒープへのポインタと長さから &str を作る */ }
}
```

「`&String` を渡す」と書いた瞬間、コンパイラが裏で `s.deref()` を呼んで `&str` に変換している。だから書く側は何も意識しなくていい。

## イメージで言うと

```
String  =「文字列を所有している人」（持ち主）
&String =「持ち主のカードを借りる」
&str    =「文字列を読む権利だけ借りる」（持ち主が誰でもいい）
```

関数が必要なのは「読む権利」だけなので、`&str` で十分。`&String` を要求するのは「持ち主が `String` の人じゃないとダメ」と狭めているのと同じ。

API 設計の原則として「必要以上に制限しない (be liberal in what you accept)」というのがあって、`&str` を取るのはこれの実践。

## 同じ哲学: `&Vec<T>` より `&[T]`

```rust
// イマイチ
fn sum(v: &Vec<i32>) -> i32 { v.iter().sum() }

// 定石
fn sum(s: &[i32]) -> i32 { s.iter().sum() }
```

`&[i32]` なら以下の全部を渡せる:

- `&Vec<i32>`（Deref coercion で `&[i32]` に）
- `&[i32; 5]`（固定長配列、Deref coercion で `&[i32]` に）
- `&[1, 2, 3]`（配列リテラル）
- 他のスライスから切り出したもの

`&Vec<i32>` だと `Vec<i32>` を持つ人しか呼べない。明らかに損。

## API を書くときの判断ツリー

```
引数で文字列をどう扱うか?
│
├─ 読むだけ                 → &str
├─ 内部に保持したい         → String     （所有権をもらう）
├─ 書き換えたい             → &mut String
└─ どっちでもいい（汎用）    → impl AsRef<str>
                              またはジェネリクス
```

スライス系も同じ:

```
引数で配列・ベクタをどう扱うか?
│
├─ 読むだけ                 → &[T]
├─ 内部に保持したい         → Vec<T>
├─ 書き換えたい             → &mut Vec<T>  （要素の追加・削除も必要なら）
│                             または &mut [T]（要素入れ替えだけなら）
```

戻り値も同じ哲学:

```
戻り値で文字列を返したい時?
│
├─ 新しく作って渡す         → String      （所有権を渡す）
├─ 引数の一部を借りて返す    → &str        （ライフタイム引き継ぎ）
└─ プログラム全体寿命の固定文字列 → &'static str
```

## なぜ「内部に保持したい」なら `String` か

参考までに、`&str` で受けて内部に保持しようとすると失敗する典型例:

```rust
struct User {
    name: String,
}

fn make_user(name: &str) -> User {
    User { name }   // ❌ expected `String`, found `&str`
}
```

直すには 2 つの方法:

```rust
// パターン A: 内部で clone
fn make_user(name: &str) -> User {
    User { name: name.to_string() }   // ヒープにコピー
}

// パターン B: 所有権をもらう
fn make_user(name: String) -> User {
    User { name }
}
```

A は呼び出し側に優しい（リテラルも渡せる）が clone のコスト。B は呼び出し側に「もう使わないから渡す」を強要するが、ゼロコピー。

「内部に保持するなら最初から `String` をもらう」が標準的な判断 ―― 結局 clone するなら呼び出し元に任せた方が無駄なコピーがない（呼び出し元は本当に必要なときだけ `String::from` する）。

## 他言語との対比

| 言語 | 文字列引数の慣習 |
|---|---|
| C | `const char*` を取るのが定石（似た思想） |
| C++ | `const std::string&` または `std::string_view` (C++17+) |
| Go | `string` を取る（コピーが安いので悩まない） |
| Java | `String` または `CharSequence`（interface で抽象化） |
| Ruby / Python / PHP | 全部参照なので何も考えない |
| Rust | `&str` を取る |

C++ の `std::string_view` は Rust の `&str` とほぼ同じコンセプト ―― 「所有しないで文字列の中身だけ参照する fat pointer」。C++ で 2017 年に追加された機能だが、Rust では最初から `&str` がこの役割。

C の `const char*` も思想は近いが、長さ情報を持っていない（ヌル終端で長さを判定）。Rust の `&str` は (ptr, len) の fat pointer なので、`O(1)` で長さが取れる。

## まとめ

- `&str` を取ると呼び出し側の自由度が広がる（`String`、リテラル、`Box<str>`、`Cow<str>` 全部 OK）
- 仕組みは Deref coercion (`String` が `Deref<Target = str>` を実装)
- 同じ哲学で `&Vec<T>` より `&[T]` を取る
- 判断ツリー: 読むだけ→ `&str`、保持→ `String`、書き換え→ `&mut String`
- API 設計の原則「必要以上に制限しない」の実践
- C++ の `string_view`、C の `const char*` と思想が近い

関連:

- [&str と String](str-vs-string.md) ―― 2 つの型の基本
- [参照と参照外し](reference-and-deref.md) ―― Deref coercion の仕組み
- [スライス](slice.md) ―― `&[T]` の正体
- [文字列リテラルと 'static](static-str.md) ―― リテラルが `&'static str` である理由
