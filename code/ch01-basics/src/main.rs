fn main() {
    // let x = 5;
    let x: i32 = 5;
    // x = 6;

    // mut
    let mut y = 5;
    println!("y初期値={y}");
    y = 6;
    println!("x={x}, y={y}");

    // シャドーイング
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    /*
     * 整数オーバーフロー
     */
    let a: u8 = 255; // 固定8bit
    println!("a={a}");
    // let b = a + 1; // releaseビルドでラップ（mod 256）されるので0になる。
    // println!("a={a}, b={b}");

    /*
     * タプル
     */
    let t: (i32, f64, &str, char, &str) = (1, 2.5, "hi", '🌀', "♨️");
    let (t1, t2, t3, t4, t5) = t;
    println!("{t1}, {t2}, {t3}, {t4}, {t5}");
    println!("{}, {}, {}, {}, {}", t.0, t.1, t.2, t.3, t.4);

    /*
     * 配列
     */
    let arr: [i32; 3] = [1, 2, 3];
    let zeros = [0u8; 16];
    println!("{}", arr.len());
    println!("{}, {}", zeros.len(), zeros[15]);

    /*
     * &strとString
     */
    let s1: &str = "hello";
    let s2: String = String::from("world");
    let s3: String = "world".to_string();
    let s4: &str = &s2;
    println!("{s1}, {s2}, {s3}, {s4}");

    /*
     * 関数
     * ref: function-return-type.md
     */
    let i1 = add1(1, 2);
    let i2 = add2(10, 20);
    let i3 = add3(11, 22);
    println!("{i1}, {i2:?}, {i3}"); //{:?} はDebug出力

    let s5 = classify(-1);
    println!("{s5}");

    /*
     * ループ
     */
    let l1 = loop {
        break 42;
    };
    println!("{l1}");

    let mut l2 = 0;
    while l2 < 3 {
        l2 += 1;
    }
    println!("{l2}");

    /*
     * イテレータ
     */
    // 5未満
    for ite1 in 1..5 {
        print!("{ite1}, ");
    }
    println!();
    // 5以下
    for ite2 in 1..=5 {
        print!("{ite2}, ");
    }
    println!();
    for ite3 in [10, 20, 30] {
        print!("{ite3}, ");
    }
    println!();

    /*
     * print系マクロ
     */
    let v = dbg!(2 + 3); // [src/main.rs:94:13] 2 + 3 = 5
    println!("{v}");

    /*
     * 型変換
     */
    let n1: i32 = 1000;
    let n2: i64 = n1 as i64;
    let n3: i64 = n1.into(); // into経由
    let n4: i32 = "42".parse().unwrap();
    //      ───   ──── ─────   ──────
    //       ①    ②     ③      ④
    // ① n の型を i32 と宣言（これが parse へのヒントになる）
    // ② &str の "42"
    // ③ parse() 呼び出し → Result<i32, ParseIntError> を返す
    // ④ unwrap() で Result から成功値の i32 を取り出す
    println!("n1={n1}, n2={n2}, n3={n3}, n4={n4}");

    /*
     * 演習
     */
    exercise_1_1();
    exercise_1_2();
    exercise_1_3();
}

fn add1(x: i32, y: i32) -> i32 {
    x + y // セミコロン無しで「式」として値を返す Rust流。
}

fn add2(x: i32, y: i32) -> () {
    let r = x + y;
    println!("{r}");
    // 空のタプルを返す
}

fn add3(x: i32, y: i32) -> i32 {
    return x + y; // add1 と同じだけれども、Rustだとadd1を好む。
}

fn classify(n: i32) -> &'static str {
    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{label}");

    if n < 0 {
        "negative" // セミコロンがないので「式」として値を返している
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

fn exercise_1_1() {
    // let x = 5;
    let mut x = 5; // mutを付けて再代入を許す。
    x = x + 1;
    // let x = x + 1; // シャドーイングで新しい束縛にする。
    println!("{x}");

    // - 同じ「概念上の値」を更新したい（カウンタ、累積値など） → mut
    // - 値を変換して別物にしたい（型を変える、加工後に上書き） → シャドーイング
}

fn exercise_1_2() {
    for i in 1..=20 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }

    // パターンマッチさせる書き方。Rustの定番。
    // match (n % 3, n % 5) {
    //     (0, 0) => String::from("FizzBuzz"),
    //     (0, _) => String::from("Fizz"),
    //     (_, 0) => String::from("Buzz"),
    //     _ => n.to_string(),
    // }
}

// cargo run --quiet -- 5
// cargo run --quiet -- 2147483646
// cargo run --quiet -- 2147483647  #overflowと出る
//  cargo run --quiet -- abc # parseに失敗しているので 0 + 1 = 1
fn exercise_1_3() {
    let s = std::env::args().nth(1).unwrap_or("0".to_string());
    let n: i32 = s.parse().unwrap_or(0);

    // let result = n.checked_add(1); // Option<i32>が返る。
    // enum Option<T> {
    //     Some(T),    // 値あり
    //     None,       // 値なし
    // }
    // これをmutch式に渡して処理。
    match n.checked_add(1) {
        Some(result) => println!("{result}"), // 値あり。
        None => println!("overflow"),         // 値なし
    };
    // Some(x) ... x という変数に束縛。後で使わないと「未使用」警告
    // Some(_x) ..._x という変数に束縛。先頭 _ で「未使用警告を抑制」
    // Some(_) ... 値を捨てる（変数として使えない）
}
