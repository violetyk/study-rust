# 解説: Non-Lexical Lifetime (NLL)

[02-ownership.md](../02-ownership.md) の NLL の補足。

> 字句的スコープから「最後の使用まで」へ。NLL はコンパイラに直感を追いつかせた

## NLL は「不便さの解決」だった

NLL は Rust 2018 エディションで導入された改善。それ以前のコンパイラは「字句的（lexical）スコープ」で借用の生存期間を判定していて、人間の直感より長く借用が生きる扱いになっていた。

つまり NLL は新しい安全性を追加した機能ではなく、「もともとコンパイラが厳しすぎたのを直した」改修。エラーが減ってコードが素直に書けるようになっただけで、安全性のルール自体は変わっていない。

## 字句的（lexical）vs 非字句的（non-lexical）

### NLL 以前: 字句的スコープ

```rust
{
    let mut s = String::from("hi");
    let r1 = &s;          // r1 の借用は…
    println!("{r1}");
    let r2 = &mut s;      // ❌ 旧仕様だとここでエラー（r1 の借用は } まで生きる扱い）
    r2.push('!');
}                         // ← 字句的には r1 もここまで生きてる扱いだった
```

「`r1` はもう使ってないんだから、`r2` を借りても安全じゃん」が直感だけど、当時のコンパイラはそれを判定する仕組みを持っていなかった。`r1` の借用は中括弧の終わりまで生きている扱い。

### NLL 以降: 最後の使用まで

```rust
let mut s = String::from("hi");
let r1 = &s;
println!("{r1}");      // ← r1 が最後に使われる場所
                        //    ここで r1 の借用は終了とコンパイラが判断
let r2 = &mut s;       // ✅ OK
r2.push('!');
```

コンパイラがコントロールフロー解析で「最後の使用箇所」を特定し、そこまでで借用を打ち切る。これが Non-Lexical Lifetime。

## 何が便利になったか

NLL のおかげで、HashMap や Vec の典型パターンが素直に書けるようになった。

### よくあるパターン: あったら使う、なかったら入れる

```rust
let mut map = std::collections::HashMap::new();
map.insert("a", 1);

if let Some(v) = map.get("a") {        // ← &map の借用が始まる
    println!("{v}");
}                                       // ← lexical だと } まで借用が続く
map.insert("b", 2);                     // ❌ lexical だとエラー、NLL なら OK
```

### ループ内での借用

```rust
let mut v = vec![1, 2, 3];
for i in 0..v.len() {
    let x = v[i];          // 借用
    println!("{x}");
}
v.push(4);                  // ✅ NLL ならループ後に借用は終わっている
```

これらが NLL 以前は「不必要にコンパイルエラーになる」コードで、ワークアラウンドとしてブロックで囲んだり、変数を分解したりが必要だった。

## エラーメッセージの読み方

Rust のエラーメッセージは NLL を前提に書かれていて、「borrow later used here」(あとでここで使ってますよ) と借用が「実質生きている範囲の終端」を教えてくれる。

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:4:14
   |
3  |     let r1 = &s;
   |              -- immutable borrow occurs here
4  |     let r2 = &mut s;
   |              ^^^^^^ mutable borrow occurs here
5  |     println!("{r1}");
   |               ---- immutable borrow later used here  ← ここで r1 を使ってるから借用がここまで生きてる
```

読み方:

1. `immutable borrow occurs here` ―― 不変借用が始まった行
2. `mutable borrow occurs here` ―― 可変借用が始まった行（エラーの主犯）
3. `immutable borrow later used here` ―― 不変借用が「最後に使われた」行（ここまで借用が生きている）

直すには:

- 3 番の使用を 2 番の前に移動する
- 3 番の使用をやめる（不要なら）

「`later used here` の場所を `mutable borrow` より前に持っていけば良い」と機械的に判断できる。

## NLL の限界（Polonius へ）

NLL も完璧ではなく、「明らかに安全なのにエラーになるパターン」がまだ残っている。代表例:

```rust
fn last_or_push<'a>(v: &'a mut Vec<i32>) -> &'a i32 {
    if let Some(last) = v.last() {     // ← v の借用
        return last;
    }
    v.push(0);                          // ❌ NLL でもエラー
    v.last().unwrap()
}
```

`return` した時点で借用は関数から「逃げる」ので、それ以降の `v.push` は安全なはずだが、現在の NLL ではこれを判別できない。

これを解決するために Polonius という次世代ボロチェッカーが開発中。2026 年現在も nightly でしか使えないが、いずれ stable に入れば「もう一段、人間の直感に近い」コンパイラになる。

Rust の借用ルールは今も進化中、という話。

## なぜ NLL が「歴史」として面白いか

ボロチェッカーは Rust の中核なので、不便でも「正しさのためのコスト」と受け入れざるを得ない部分があった。NLL は「正しさを保ったまま、不便さだけ削った」改善で、Rust が成熟していくプロセスの代表例。

- 安全性ルール（borrow rules）はそのまま
- 安全性を判定するコンパイラの精度だけが向上
- 過去に書いたコードは壊れない（より緩くなる方向の変更）

「言語の進化 = ルール追加」ではなく「言語の進化 = 制約を緩める」というパターンもある、というのは設計の観点で示唆深い。

## まとめ

- NLL は「字句的スコープ → 最後の使用まで」への改善（2018 年導入）
- 安全性ルールは変えず、コンパイラの判定精度を上げた
- HashMap / Vec の典型パターンが素直に書けるようになった
- エラーメッセージの `later used here` が借用の実質的な終端
- まだ限界はあり、Polonius で更なる改善が進行中

関連:

- [借用ルール vs SQL の MVCC](borrow-rules-and-mvcc.md) ―― 借用ルール自体の意味
- [所有のラインは 1 本だけ](move-and-ownership-line.md) ―― 借用の前提となる所有権の話
