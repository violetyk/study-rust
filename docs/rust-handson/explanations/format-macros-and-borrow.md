# 解説: `println!` と借用

[02-ownership.md](../02-ownership.md) の演習 2-1 で気づくポイントの補足。

> `println!` は関数じゃなくてマクロ。だから所有権を奪わず黙って借用する

## 出発点の疑問

```rust
let s = String::from("hi");
println!("{}", s);     // ✅
println!("{}", &s);    // ✅ こっちでも動く

println!("{}", s);     // ✅ 何回でも書ける（s が move されない）
```

`s` と書いても `&s` と書いても両方動く。しかも何回 `println!` を呼んでも `s` は使える ―― これはなぜか?

## 仕組み 1: `println!` はマクロで引数を借用する

`println!("{}", s)` は、おおまかにこう展開される:

```rust
::std::io::_print(format_args!("{}\n", s));
```

ポイントは `format_args!` が引数を借用すること。つまり `s` と書いても、マクロが裏で `&s` 相当に変換している。だから所有権は move されず、`s` はマクロ実行後も使える。

これは関数呼び出しとは決定的に違う:

```rust
fn show(x: String) { println!("{x}"); }   // x を move で受け取る関数

let s = String::from("hi");
show(s);              // ← ここで s の所有権が move
// println!("{s}");   // ❌ もう使えない

let s = String::from("hi");
println!("{s}");      // ← マクロだから move しない
println!("{s}");      // ✅ まだ使える
```

普通の関数なら `String` を渡したら move されるが、`println!` は内部で借用するからセーフ。

## 仕組み 2: `Display` トレイトは「`T` も `&T` も」受け付ける

標準ライブラリには便利な実装がある:

```rust
// 疑似コード (実際は core::fmt にある)
impl<T: Display + ?Sized> Display for &T {
    fn fmt(&self, f: &mut Formatter) -> Result {
        (**self).fmt(f)   // 参照外しして本体の Display を呼ぶ
    }
}
```

「`T` が `Display` なら、`&T` も自動で `Display`」。だから:

- `println!("{}", s)` → `String` の `Display` が呼ばれる
- `println!("{}", &s)` → `&String` の `Display` が呼ばれる（中で `*` して `String` の Display に委譲）

どっちでも最終的に出力される文字列は同じ。

## 全体の流れ

```
println!("{}", s)
  ↓ マクロ展開
format_args!("{}", s)
  ↓ マクロが引数を借用
内部的に &s として扱う
  ↓ Display::fmt(&self, ...) が呼ばれる
"hi" が出力される
```

```
println!("{}", &s)
  ↓ マクロ展開
format_args!("{}", &s)
  ↓ マクロが引数を借用
内部的に &&s として扱う
  ↓ Display::fmt(&self, ...) が呼ばれる
  ↓ &T の Display 実装が、内部で *self（つまり &s）に委譲
  ↓ さらに &String の Display が String の Display に委譲
"hi" が出力される
```

両方とも所有権を奪わず、同じ出力になる。

## 関数 vs マクロの違い

| | 関数 (`fn show(x: String)`) | マクロ (`println!("{}", ...)`) |
|---|---|---|
| 引数の受け取り方 | 所有権で受け取る（move） | 内部で借用（move しない） |
| `String` を渡すと | 元の変数は使えなくなる | 元の変数は引き続き使える |
| `&String` を渡すと | 型エラー | OK（借用なので何でも来い） |
| `&str` リテラルを渡すと | 型エラー | OK |
| 呼び出し側の負担 | 「もう要らない」を明示する必要あり | 何も考えなくていい |

`println!` がマクロな理由は、もし関数だったら「`fn println(fmt: &str, x: &dyn Display)`」みたいなシグネチャになって、複数の引数を任意の型で取れない（Rust には可変長引数の関数がない）。マクロなら任意の数・任意の型の引数を取れる。

## なぜこの設計か

「フォーマット出力したいだけなのに所有権を渡すのおかしいよね」を自動解決している。

もし `println!` が普通の関数だったら、毎回こう書く必要があった:

```rust
let s = String::from("hi");
println!("{}", &s);          // 毎回 & が必要
println!("{}", &s);          // しかも毎回書かないと move される
let owned = s.clone();       // clone も発生
```

ありえない書き味。マクロ化することで「使う側が借用を意識しないで書ける」を実現している。

副産物として、`{}` で表示できる型なら何でも渡せる、`{:?}` で `Debug` も同じ仕組みで動く、`{x}` で変数キャプチャもできる、と機能が広がっている。

## 他のフォーマットマクロも同じ

借用の仕組みは標準のフォーマットマクロ全部に共通:

| マクロ | 用途 |
|---|---|
| `println!` | stdout に改行付き出力 |
| `print!` | stdout に出力（改行なし） |
| `eprintln!` | stderr に改行付き出力 |
| `eprint!` | stderr に出力（改行なし） |
| `format!` | 文字列を返す（`String`） |
| `write!` / `writeln!` | 任意の `Write` トレイト実装先に書き込む |

全部、引数を借用するので move しない。これらは内部で `format_args!` を共通利用している。

## `Debug` トレイトと `{:?}` も同じ

```rust
let v = vec![1, 2, 3];
println!("{:?}", v);   // [1, 2, 3]
println!("{:?}", v);   // ✅ 何回でも書ける
```

`Debug` トレイトも `&self` を受け取る実装で、`println!` がマクロで借用するから、`Vec<T>` を渡しても move されない。

## Rust 2021 の捕捉識別子 (captured identifier)

Rust 2021 エディションから、波括弧の中に変数名を直接書ける:

```rust
let s = String::from("hi");
println!("{s}");        // ✅ Rust 2021+
println!("{}", s);      // ✅ どちらも同じ意味
```

これも内部的には `format_args!` が `s` を借用するので、move しない。

ただし式は書けない:

```rust
let v = vec![1, 2, 3];
println!("{v[0]}");           // ❌
println!("{}", v[0]);         // ✅ こっちで書く
```

## 他言語との対比

| 言語 | 出力の仕組み | 所有権・参照の扱い |
|---|---|---|
| C | `printf` (可変長引数関数) | プリミティブはそのまま、文字列はポインタ |
| C++ | `std::cout <<` (演算子オーバーロード) | 参照渡し |
| Go | `fmt.Println` (可変長引数 + interface) | 値コピー (string は安い) |
| Java | `System.out.println` (オーバーロード) | 参照（全部参照だから意識しない） |
| Python | `print` (関数) | 参照 |
| Rust | `println!` (マクロ + format_args!) | 借用（move しない） |

Rust 以外は「参照渡しまたは値コピー」が普通で、所有権の概念がそもそもない。Rust だけ「マクロにして借用を強制する」という一見トリッキーな方法を採っているが、これは所有権を持つ言語ならではの工夫。

## まとめ

- `println!` はマクロで、`format_args!` を内部で使って引数を借用する → 所有権を奪わない
- `s` と書いても `&s` と書いても両方動く（マクロ + Display の `&T` 実装の合わせ技）
- 関数なら move されるが、マクロは借用するので何回でも呼べる
- 他のフォーマットマクロ (`format!`, `eprintln!`, `write!` など) も同じ仕組み
- `Debug` (`{:?}`) も同じ
- Rust 2021 の `{s}` も同じく借用なので move しない
- マクロにした理由は「フォーマット出力で所有権を意識させないため」

関連:

- [参照と参照外し](reference-and-deref.md) ―― 自動 deref / Deref coercion の話
- [所有のラインは 1 本だけ](move-and-ownership-line.md) ―― move と借用の区別
- [Copy と Clone](copy-and-clone.md) ―― 関数に渡す時の move/copy 判定
