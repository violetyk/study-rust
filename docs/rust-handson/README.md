# Rust ハンズオンチュートリアル

PHP / Go / Python / Ruby の経験者が、手を動かしながら Rust 特有の機能を理解するためのチュートリアル。

## このチュートリアルの方針

- 「変数とは」のような基礎は省略し、Rust 特有の概念に集中する
- 既知の言語（特に Go）との対比で説明する
- 各章で実際に `cargo new` してコードを書き、`cargo run` で動かす
- 各章末に「演習」と「落とし穴」を置く。手を動かして理解を確認する
- コンパイルエラーを意図的に出して、ボローチェッカーが何を防いでいるかを体感する

## 学習の進め方

1. 章ごとにディレクトリを作って `cargo new` する
2. 説明を読みながら `src/main.rs` に写経する
3. 演習を実装する
4. 落とし穴のコードを試して、コンパイラのエラーメッセージを読む
5. チェックリストを確認して次の章へ

## 目次

⭐ は特につまずきやすい・時間をかける価値がある章。

| #  | タイトル | キーワード | 想定時間 | コメント |
|----|---------|-----------|---------|---------|
| 00 | [環境構築と Cargo](00-setup.md) | rustup, cargo, fmt, clippy | 15〜30分 | rustup 入れてビルドが通るまで |
| 01 | [基本構文](01-basics.md) | 不変デフォルト, 式指向, シャドーイング | 30〜45分 | 他言語経験者なら速い |
| 02 | [所有権と借用](02-ownership.md) ⭐ | move, borrow, &mut | 1.5〜2時間 | Rust最大の壁。コンパイラと対話する時間 |
| 03 | [構造体と enum](03-structs-enums.md) | struct, enum, match | 45分〜1時間 | パターンマッチで少し時間 |
| 04 | [エラーハンドリング](04-error-handling.md) | Result, Option, ?, anyhow | 1時間 | `?` 演算子と anyhow に慣れる |
| 05 | [トレイトとジェネリクス](05-traits-generics.md) | trait, impl, ジェネリック境界 | 1〜1.5時間 | dyn と impl の使い分けで詰まる |
| 06 | [コレクションとイテレータ](06-collections-iterators.md) | Vec, HashMap, iter | 45分〜1時間 | `into_iter`/`iter`/`iter_mut` の差で混乱 |
| 07 | [ライフタイム](07-lifetimes.md) ⭐ | 'a, 省略規則 | 1〜1.5時間 | 二番目の壁 |
| 08 | [モジュールとクレート](08-modules-crates.md) | mod, pub, workspace | 30〜45分 | Goのpackage理解があれば速い |
| 09 | [テストとドキュメント](09-testing.md) | #[test], doctest | 30〜45分 | doctest が新鮮 |
| 10 | [並行プログラミング](10-concurrency.md) | thread, Send/Sync, channel | 1〜1.5時間 | Send/Sync の理解 |
| 11 | [非同期プログラミング](11-async.md) | async/await, tokio | 1〜1.5時間 | tokio セットアップ込み |
| 12 | [ミニプロジェクト: 簡易 grep](12-cli-project.md) | 集大成 | 2〜3時間 | これまでの章を総動員 |

合計の目安:

- ⏱ 通読＋手を動かす: 12〜16時間
- 🚀 写経メインで駆け抜ける: 6〜8時間
- 🧠 演習を全部解いて納得しながら: 20時間+

平日30分〜1時間で2〜3週間、週末まとめてやるなら2〜3週末で一周できる規模感。02 と 07 は意図的に時間をかけて、コンパイラと喧嘩するのが習得の近道。

## ディレクトリ構成（推奨）

各章のコードはこのディレクトリ配下に作るのが楽。

```
study-rust/
├── docs/
│   └── rust-handson/
│       ├── README.md
│       ├── 00-setup.md
│       ├── 01-basics.md
│       └── ...
└── code/
    ├── ch01-basics/      ← cargo new でここに作る
    ├── ch02-ownership/
    └── ...
```

## 既知言語との対応マップ

ざっくりした対応関係。詳細は各章で補足する。

| 概念       | Rust                | Go                | PHP / Ruby               |
| -------- | ------------------- | ----------------- | ------------------------ |
| 可変変数     | `let mut x`         | `x := ...`        | デフォルト可変                  |
| 定数       | `let x` / `const X` | `const X`         | `const`                  |
| インターフェース | `trait`             | `interface`       | `interface` / `module`   |
| エラー      | `Result<T, E>`      | `(T, error)`      | 例外                       |
| null     | `Option<T>`         | `*T` / zero value | `null`                   |
| パッケージ    | `crate` / `mod`     | `package`         | `namespace` / `gem`      |
| 並行       | `thread` / `tokio`  | `goroutine`       | (実用上は別プロセス)              |
| ジェネリクス   | `<T: Bound>`        | `[T any]`         | (PHP は最近、Ruby はダックタイピング) |
| GC       | なし（所有権）             | あり                | あり                       |

## 参考資料

- 公式: <https://doc.rust-lang.org/book/> （The Rust Programming Language、通称「The Book」）
- 演習: <https://github.com/rust-lang/rustlings>
- API: <https://doc.rust-lang.org/std/>
- クレート検索: <https://crates.io/>

## このチュートリアル独自の表記

- ✅ ... コンパイル・実行できる
- ❌ ... わざとコンパイルエラーになる例（学習用）
- ⚠️ ... 動くが推奨されない、または落とし穴
- 📝 ... 演習問題
