# 03. 構造体と enum

## 学習目標

- `struct` を定義してメソッドを生やせる
- `enum` で代数的データ型を表現できる
- `match` で網羅的にパターンマッチできる
- DDDの値オブジェクト・エンティティを Rust でどう書くかの感覚を掴む

EM／DDD 経験者なら、`struct` + `impl` + `enum` の組み合わせは「型でドメインを表現する」のに非常に強力なことが分かるはず。Go の構造体 + メソッド + 型エイリアスより表現力が高い。

## プロジェクト

```bash
cd code
cargo new ch03-structs-enums
cd ch03-structs-enums
```

## struct の 3 種類

```rust
// 1. 名前付きフィールド（一番よく使う）
struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

// 2. タプル構造体（フィールド名なし。新しい型を作る軽量手段）
struct UserId(u64);
struct Email(String);

// 3. ユニット構造体（フィールドなし。マーカー型）
struct EndOfStream;
```

タプル構造体は「プリミティブ型を意味のある型に閉じ込める」のに使う（newtype パターン）。`UserId` と `OrderId` を間違えて代入できなくなる。DDD の値オブジェクトと相性が良い。

## インスタンス化と更新

```rust
let u = User {
    id: 1,
    name: String::from("Yuhei"),
    email: String::from("yuhei@example.com"),
    active: true,
};

// フィールド名と変数名が同じなら省略できる（shorthand）
fn build(name: String, email: String) -> User {
    User { id: 0, name, email, active: true }
}

// 構造体更新記法（残りを別インスタンスからコピー）
let u2 = User { name: String::from("Taro"), ..u };
// 注意: u の name 以外がムーブされるので、u はもう使えない（中身に Copy でない型があるため）
```

## メソッドと関連関数: `impl` ブロック

```rust
impl User {
    // メソッド（&self を取る）
    fn display(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }

    // 可変メソッド
    fn deactivate(&mut self) {
        self.active = false;
    }

    // 所有権を奪うメソッド（消費する操作）
    fn into_email(self) -> String {
        self.email
    }

    // 関連関数（Go の static method 相当。インスタンスを取らない）
    fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email, active: true }
    }
}
```

| 受け取り方 | 役割 |
|----------|-----|
| `&self` | 読み取り |
| `&mut self` | 可変参照（変更） |
| `self` | 所有権を奪う（消費する操作） |

`Self`（大文字）は「この型」のエイリアス。`User` と書く代わりに `Self` を使うのがイディオム。

呼び出し:

```rust
let mut u = User::new(1, "Yuhei".into(), "y@example.com".into());
println!("{}", u.display());
u.deactivate();
let email = u.into_email();   // u はもう使えない
```

## 派生（derive）

`#[derive(...)]` で代表的なトレイトを自動実装できる。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserId(u64);

let id = UserId(1);
println!("{:?}", id);     // Debug: UserId(1)
println!("{:#?}", id);    // 整形 Debug
let id2 = id.clone();
assert_eq!(id, id2);
```

主要な derive:

| トレイト | 効果 |
|---------|-----|
| `Debug` | `{:?}` で表示できる |
| `Clone` | `.clone()` できる |
| `Copy` | スタックコピー（条件あり）|
| `PartialEq` / `Eq` | `==` / `!=` |
| `PartialOrd` / `Ord` | `<` `>` `cmp` |
| `Hash` | `HashMap` のキーにできる |
| `Default` | `Default::default()` で初期値 |

「とりあえず `Debug` だけは付けておく」のが定石。

## enum: 代数的データ型

C の enum と違い、各バリアントがデータを持てる。これが超強力。

```rust
enum Shape {
    Circle { radius: f64 },                    // 構造体風
    Rectangle(f64, f64),                        // タプル風
    Square(f64),
    Point,                                      // データなし
}
```

DDD でいう「Sum Type（直和型）」で、状態遷移や種類違いを表現するのに最適。

```rust
enum OrderStatus {
    Pending,
    Paid { paid_at: u64 },
    Shipped { tracking_id: String },
    Delivered { delivered_at: u64 },
    Cancelled { reason: String },
}
```

`Paid` の状態にしかないデータ（決済日時）を `OrderStatus` に内包できる。Go なら別フィールドにするか、別 struct を使い分けることになるが、Rust ならコンパイラが「Paid の時だけ paid_at にアクセスできる」を保証する。

## match: 網羅的パターンマッチ

```rust
fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(w, h) => w * h,
        Shape::Square(s) => s * s,
        Shape::Point => 0.0,
    }
}
```

`match` には網羅性チェックがある。バリアントを 1 つでも漏らすとコンパイルエラー。新しいバリアントを追加したら、match 全箇所が壊れて教えてくれる（リファクタリングの強力な味方）。

### ガード

```rust
let n = 5;
match n {
    x if x < 0 => println!("negative"),
    0 => println!("zero"),
    x if x % 2 == 0 => println!("positive even"),
    _ => println!("positive odd"),
}
```

`_` はワイルドカード（その他全部）。

### 範囲・複数パターン

```rust
match c {
    'a'..='z' => println!("lower"),
    'A'..='Z' => println!("upper"),
    '0' | '1' | '2' => println!("low digit"),
    _ => println!("other"),
}
```

### 分解（destructuring）

```rust
let pt = (3, 4);
let (x, y) = pt;

let User { name, email, .. } = u;   // 必要なフィールドだけ取り出す
```

## `if let` と `let else` と `while let`

特定のパターンだけ扱いたいときの省略形。

```rust
let some_value = Some(5);

// match の代わりに if let
if let Some(x) = some_value {
    println!("got {x}");
}

// let else: 失敗時に早期 return / break / panic できる
fn get_age(s: Option<&str>) -> u32 {
    let Some(s) = s else {
        return 0;
    };
    s.parse().unwrap_or(0)
}

// while let: ループ
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{top}");
}
```

`let else` は Go の `if v, ok := m[k]; !ok { return }` のようなパターンを綺麗に書ける。

## DDD 的な使い方の例

商品 ID / 数量 / 金額を newtype で表す:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Sku(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Quantity(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PriceJpy(u64);

impl PriceJpy {
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

#[derive(Debug)]
struct LineItem {
    sku: Sku,
    quantity: Quantity,
    unit_price: PriceJpy,
}

impl LineItem {
    fn subtotal(&self) -> PriceJpy {
        PriceJpy(self.unit_price.0 * self.quantity.0 as u64)
    }
}
```

`Sku` と `PriceJpy` を取り違えてもコンパイルが通らない。Go の `type Sku uint64` より厳密。

## 演習

📝 **演習 3-1**: 上の `OrderStatus` enum を定義し、現在の状態から「キャンセル可能か」を返す `fn can_cancel(&self) -> bool` を `match` で実装せよ。`Pending` と `Paid` のみ true、`Shipped` 以降は false。

📝 **演習 3-2**: 信号機を表す `enum Signal { Red, Yellow, Green }` を定義し、`fn next(&self) -> Signal` を実装。`Red → Green → Yellow → Red` の順で遷移させる。`#[derive(Debug, PartialEq)]` を付ける。

📝 **演習 3-3**: 二分木を enum で表現せよ。

```rust
enum Tree {
    Leaf,
    Node {
        value: i32,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}
```

`fn sum(&self) -> i32` を実装。なぜ `Box<Tree>` なのか考えよ（ヒント: サイズ無限大）。

## チェックリスト

- [ ] `&self` / `&mut self` / `self` を使い分けられる
- [ ] `enum` のバリアントにデータを持たせて使える
- [ ] `match` の網羅性チェックの恩恵を体感した
- [ ] newtype パターンの利点が言える
- [ ] `Box<T>` がなぜ enum の再帰に必要か説明できる

## 落とし穴

⚠️ **`derive(Copy)` は条件付き**: 全フィールドが Copy な場合のみ。`String` を含む struct には付けられない。

⚠️ **構造体更新記法は move する**: `..u` は残りフィールドを move する。Copy でない型を含む場合、元の `u` は使えなくなる。

⚠️ **`enum` の再帰には `Box`**: そのままだと「サイズが決まらない」。`Box<T>` でヒープ参照にしてサイズを固定する。

⚠️ **`match` の `_` を安易に使わない**: 網羅性チェックの恩恵を捨てることになる。明示的に書く方が、enum 追加時に教えてくれる。

⚠️ **`Self` と `self` は別物**: 大文字 `Self` は型、小文字 `self` はインスタンス。
