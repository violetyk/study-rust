# 01. 基本構文

## 学習目標

- 変数のデフォルトが「不変」であることに慣れる
- 「式指向」（ほぼ全てが値を返す）の感覚を掴む
- プリミティブ型と整数オーバーフローの扱いを理解する
- `if` `loop` `while` `for` の書き方を覚える

Rust は「不変がデフォルト」「式指向」「強い静的型付け」の 3 点が他言語と一番違う。

## プロジェクトを作る

```bash
cd code
cargo new ch01-basics
cd ch01-basics
```

以降、`src/main.rs` に書いていく。

## 変数: 不変がデフォルト

```rust
fn main() {
    let x = 5;
    // x = 6;        // ❌ コンパイルエラー: cannot assign twice to immutable variable
    let mut y = 5;
    y = 6;           // ✅ mut を付ければ再代入できる
    println!("x={x}, y={y}");
}
```

| 言語 | デフォルト | 不変宣言 |
|-----|---------|---------|
| Rust | 不変 | `let` |
| Go | 可変 | `const`（コンパイル時定数のみ） |
| PHP / Ruby | 可変 | (なし、慣習) |

「変えないものは変えない」を強制される。並行プログラミングで効いてくる。

## シャドーイング

同じ名前で `let` を重ねると、新しい変数で前を覆い隠せる。型を変えてもよい。

```rust
let spaces = "   ";
let spaces = spaces.len();   // ✅ &str → usize に「型を変えながら」上書き
println!("{spaces}");        // 3
```

これは `mut` とは違う。`mut` は同じ変数の値を変える、シャドーイングは新しい変数を束縛する。スコープを抜ければ元に戻る。

詳しく: [変数・値・束縛、mut とシャドーイング](explanations/binding-and-shadowing.md)

## 定数

```rust
const MAX_RETRY: u32 = 3;
```

- 型注釈が必須
- グローバルにも書ける
- コンパイル時に評価される（`const fn` で定義された関数なら呼べる）

## プリミティブ型

| 種類 | 型 |
|-----|---|
| 符号付き整数 | `i8 / i16 / i32 / i64 / i128 / isize` |
| 符号なし整数 | `u8 / u16 / u32 / u64 / u128 / usize` |
| 浮動小数 | `f32 / f64` |
| 真偽値 | `bool` |
| 文字 | `char`（4 バイト Unicode スカラー値） |
| 単位型 | `()` （Go の空構造体相当） |

整数のデフォルトは `i32`、浮動小数は `f64`。配列添字や長さは `usize`。

```rust
let a: i64 = 1_000_000_000_000;   // _ は読みやすさのための区切り
let b = 1_000u32;                 // リテラルに型サフィックスも付けられる
let c = 0xff;                     // 16進
let d = 0b1010;                   // 2進
let e: f64 = 2.5;
```

### isize と usize

固定幅の `i8 / i16 / i32 / ...` と違って、`isize / usize` は実行環境のポインタ幅に合わせて変わる（64bit システムなら 64bit、32bit システムなら 32bit）。

配列の長さや添字は「メモリ上のオフセット」なので、メモリアドレスと同じ幅じゃないと表現しきれない。32bit システムではアドレスが 32bit までなので、長さや添字もそれに合わせる必要がある。だから Rust は `Vec::len()` の戻り値や `vec[i]` の添字を `usize` で固定している。

現代の Mac / Linux はほぼ 64bit なので、実質 `usize == u64` のサイズだと思って良い（Apple Silicon、Intel Mac (x86_64)、Linux x86_64 すべて 64bit）。

C / C++ の `size_t` / `ssize_t`、Go の `int` / `uint` と同等の概念。

### 整数オーバーフロー

⚠️ debug ビルドではパニック、release ビルドではラップする（モジュロ演算）。地味な落とし穴。

```rust
let x: u8 = 255;
let y = x + 1;   // debug: panic / release: 0
```

明示的に挙動を選ぶメソッド:

- `checked_add` → `Option<T>`（あふれたら `None`）
- `wrapping_add` → ラップ
- `saturating_add` → 上限/下限で止める
- `overflowing_add` → `(T, bool)` を返す

金額計算など重要なロジックは必ず `checked_*` を使う。

詳しく: [Option<T>](explanations/option.md)

## タプルと配列

```rust
let t: (i32, f64, &str) = (1, 2.5, "hi");
let (a, b, c) = t;             // 分解
println!("{}", t.0);           // インデックスでもアクセス可

let arr: [i32; 3] = [1, 2, 3]; // 固定長
let zeros = [0u8; 16];          // [0, 0, ..., 0]
println!("{}", arr.len());     // 3
```

可変長は次章以降の `Vec<T>` を使う。

## 文字列: `&str` と `String`

最初に詰まりやすい。詳しくは 06 章。ここでは違いだけ。

| 型 | 何者 | 例 |
|---|------|---|
| `&str` | 文字列スライス（参照） | `"hello"` リテラル |
| `String` | ヒープ確保された可変長文字列 | `String::from("hello")` |

```rust
let s1: &str = "hello";              // バイナリに埋め込まれた静的データへの参照
let s2: String = String::from("hi"); // ヒープに確保
let s3: String = "hi".to_string();   // 同等
let s4: &str = &s2;                  // String → &str への参照取得（よく使う）
```

PHP/Go でいう「文字列」感覚で `&str` だけ使うとすぐ詰まる。「持ち主は誰か」を意識し始めるのが Rust への第一歩。

詳しく: [&str と String の違い](explanations/str-vs-string.md) ｜ [スタックとヒープ](explanations/stack-and-heap.md) ｜ [スライスとは](explanations/slice.md)

## 関数

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y    // セミコロンなし → 式として値を返す
}
```

- 仮引数の型は必須
- 戻り値の型は `->` で書く。`()` 戻りは省略可
- 関数の最後の式が戻り値（`return` も書けるが省略がイディオム）

詳しく: [関数の戻り値の型はどこから来るか](explanations/function-return-type.md) ｜ [&'static str と文字列リテラルの正体](explanations/static-str.md)

```rust
fn classify(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}
```

`if` 自体が式なので、変数に代入できる。三項演算子はない（`if` で十分なため）。

```rust
let label = if n % 2 == 0 { "even" } else { "odd" };
```

⚠️ `if` を式として使うとき、各分岐は同じ型でなければならない。

## ループ

```rust
// 無限ループ
loop {
    break;
}

// 値を返す loop
let v = loop {
    break 42;          // ← 値を返せる
};

// while
let mut i = 0;
while i < 3 {
    i += 1;
}

// for（イテレータベース。ほぼこれを使う）
for x in 1..=5 {       // 1, 2, 3, 4, 5
    println!("{x}");
}
for x in [10, 20, 30] {
    println!("{x}");
}
```

C の `for(int i=0;...)` 形式はない。レンジかイテレータを使う。

| 範囲 | 意味 |
|-----|-----|
| `0..n` | 0 以上 n 未満（半開区間） |
| `0..=n` | 0 以上 n 以下（閉区間） |

## print 系マクロ

`println!` `print!` `eprintln!` `format!` `dbg!` を覚えておく。すべてマクロ（末尾の `!`）。

```rust
let name = "Yuhei";
let age = 44;
println!("name={name}, age={age}");           // 名前付きキャプチャ（推奨）
println!("name={}, age={}", name, age);       // 位置指定
println!("hex={:x}, pad={:>5}", 255, age);    // フォーマット指定
eprintln!("error log");                       // stderr へ
let s = format!("{name}-{age}");              // 文字列を作る
let v = dbg!(2 + 3);                          // ファイル:行 と式を stderr に出して値を返す
```

`dbg!` はデバッグの強い味方。`println!` で囲むよりずっと便利。

## 型変換: `as` と `From / Into`

Rust は暗黙の数値変換をしない。明示的に書く。

```rust
let n: i32 = 1000;
let m: i64 = n as i64;       // プリミティブ間は as
let m2: i64 = n.into();      // From/Into 経由（こちらが安全）

// 文字列→数値
let n: i32 = "42".parse().unwrap();   // 失敗するかも → Result が返る
```

型推論が効く場面では型注釈を省けるが、`parse` のように戻り型が決まらないものは注釈が必要。

詳しく: [parse / Result / unwrap](explanations/parse-and-result.md)

## 演習

📝 **演習 1-1**: 次のコードを書いて、なぜコンパイルエラーになるかを読む。直してから実行する。

```rust
fn main() {
    let x = 5;
    x = x + 1;
    println!("{x}");
}
```

📝 **演習 1-2**: 整数 `n: i32` を引数に取り、FizzBuzz の文字列を返す関数 `fizzbuzz(n: i32) -> String` を書いて、`for` で 1..=20 を回して表示する。`if` を式として使ってみる。

📝 **演習 1-3**: ユーザー入力（`std::env::args` の 1 番目）を `i32` にパースし、`checked_add` で 1 を足してみる。`None` のときは "overflow" と出力する。

```rust
let s = std::env::args().nth(1).unwrap_or("0".to_string());
let n: i32 = s.parse().unwrap_or(0);
// この続きを書く
```

## チェックリスト

- [x] `let` と `let mut` の違いが言える
- [x] シャドーイングと `mut` の違いが言える
- [x] `if` が式である意味を体感した
- [x] `&str` と `String` の違いを（とりあえず）言える
- [x] 整数オーバーフローの挙動と対処法を知っている
- [x] `dbg!` を実際に使った

## 落とし穴

⚠️ **`==` の左右の型は厳密に同じ**: `let n: i64 = 0; if n == 0 { ... }` は OK だが、`if n == 0i32` は型不一致。

⚠️ **`println!` はマクロ**: `!` を忘れると関数呼び出しと解釈されてエラー。

⚠️ **インクリメント `++` がない**: `i += 1` を使う。

⚠️ **改行は `print!` では出ない**: `println!` を使うか `\n` を入れる。

⚠️ **return 不要**: 関数末尾は式として戻り値になる。`return x;` を書くとセミコロンが必要。慣れるまで混乱する。
