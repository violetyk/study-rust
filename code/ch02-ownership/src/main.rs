fn main() {
    exercise_2_1_move();
    exercise_2_1_borrow();
    exercise_2_1_clone();
    exercise_2_2();
}

// 演習 2-1 まとめ: 3 パターンの比較
//
//   move を返す:
//     - 関数のシグネチャ変更: 必要（タプル返却）
//     - 呼び出し側の手間:     受け取り直し
//     - コスト:               ゼロ（move）
//
//   借用:
//     - 関数のシグネチャ変更: 必要（&str 受け取り）
//     - 呼び出し側の手間:     & 付けるだけ
//     - コスト:               ゼロ（参照）
//
//   clone:
//     - 関数のシグネチャ変更: 不要
//     - 呼び出し側の手間:     .clone() 付ける
//     - コスト:               ヒープ複製
//
// → 借用が一番便利、書き味もコストも最小

// 所有権がmoveするパターン
// タプルで返さないと使い物にならないので不便名子とが分かる
fn exercise_2_1_move() {
    let s = String::from("move");
    // sをmove, 戻り値で受け取り直す
    // sで受け取っているが、同じ名前で別変数を新しく作っている、シャドーイング。
    let (s, len) = compute_len_move(s);
    println!("{} の長さは {}", s, len);
    //println!("{} の長さは {}", &s, len);
}

fn compute_len_move(s: String) -> (String, usize) {
    let len = s.len();
    (s, len) // 所有権を呼び元に戻す
}

// 所有権を借りるパターン
fn exercise_2_1_borrow() {
    let s = String::from("borrow");
    let len = compute_len_borrow(&s);
    println!("{} の長さは {}", s, len);
}

fn compute_len_borrow(s: &str) -> usize {
    // &strとして、 借用
    return s.len();
}

// clone()で呼び出し側で先に複製を作って、複製の方を渡すパターン
fn exercise_2_1_clone() {
    let s = String::from("clone");
    // clone()で新しい String（ヒープ全体をコピー）を作り、渡す
    let len = compute_len_clone(s.clone());
    // sは呼び出し側に残ったままなのでprintln!できる
    println!("{} の長さは {}", s, len);
}

fn compute_len_clone(s: String) -> usize {
    return s.len();
}

fn exercise_2_2() {
    let v = vec![1, 2, 3];
    let (total, max) = total_and_max(&v);
    println!("{total}, {max}");
    println!("{:?}", v); // vがまだ使える
}

// fn total_and_max(v: Vec<i32>) -> (i32, i32) {
fn total_and_max(v: &[i32]) -> (i32, i32) {
    // Option<&i32> の中身は &i32（参照）なので、* で参照外しするか、.copied() で値にコピーするかが必要:
    (v.iter().sum(), *v.iter().max().unwrap())
}
