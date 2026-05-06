# study-rust

PHP / Go / Python / Ruby 経験者向けの Rust ハンズオン学習リポジトリ。個人学習記録。

「立ち止まって深掘り」を学習スタイルにしている。気になった概念は本流の章を進める前に [`docs/rust-handson/explanations/`](docs/rust-handson/explanations/) に切り出して、後から戻れるようにする。

## Claude Code 伴走スタイル

このリポジトリは [Claude Code](https://docs.claude.com/en/docs/claude-code) を起動して、伴走してもらいながら学習を進めている。

- 「これは何？」と聞くとその場で詳しく答えてくれるので、引っかかった概念を即座に深掘りできる
- 会話で消えずに残したい内容は `docs/rust-handson/explanations/` に Markdown として書き出していく
- 演習を実装したらコードレビューしてもらう（clippy の指摘も一緒に読む）
- 章を閉じる手順（チェックリスト反映、コミット、push）まで含めて伴走

Claude Code 向けの運用ルールは [CLAUDE.md](CLAUDE.md) を参照。

## ディレクトリ構成

```
study-rust/
├── README.md
├── CLAUDE.md
├── docs/
│   └── rust-handson/
│       ├── README.md           本流の章インデックス
│       ├── 00-setup.md         環境構築と Cargo
│       ├── 01-basics.md        基本構文
│       ├── 02-ownership.md     所有権
│       ├── ...
│       ├── 12-cli-project.md   CLI プロジェクト
│       └── explanations/       学習中の疑問を切り出した補足解説
│           ├── binding-and-shadowing.md
│           ├── option.md
│           ├── slice.md
│           └── ...
└── code/
    ├── ch00-hello/             章 00 の演習
    ├── ch01-basics/            章 01 の演習
    └── ...
```

## 進め方

1. [`docs/rust-handson/00-setup.md`](docs/rust-handson/00-setup.md) から順に読む
2. `code/chXX-...` に演習プロジェクトを `cargo new` で作る
3. 章を読みながら `src/main.rs` で例を試す、演習を解く
4. 詰まった概念があれば `explanations/` の該当ファイルを読む
5. 章末のチェックリストを埋めて、コミット → push でクローズ

## 学習スタイル

このリポジトリは「立ち止まって深掘り」スタイルを前提にしている。

- 「これなんだろう？」が出てきたらその場で深掘りする
- 短期的には進みが遅く見えるが、後の章で同じ概念が再登場した時に戻れる場所がある
- 深掘り内容は学習資産として `explanations/` に蓄積する
- 「先に進む」より「腑に落ちる」を優先する

特に Rust は概念が積み上がる言語（プリミティブ → 所有権 → 借用 → ライフタイム → トレイト → 非同期 と一本道で繋がる）なので、土台が曖昧だと後半でしんどくなる。逆に序盤で時間をかけると、後半で「あ、これ知ってる」が増える。

## さらに詳しく

`explanations/` の運用ルール、コードの慣習、章を閉じる手順などは [CLAUDE.md](CLAUDE.md) を参照。
