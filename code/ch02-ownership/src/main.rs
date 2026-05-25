fn main() {
    exercise_2_1_move();
}

fn exercise_2_1_move() {
    let s = String::from("hi");
    let len = compute_len(&s);
    // println!("{} の長さは {}", s, len);
    println!("{} の長さは {}", &s, len);
}

fn compute_len(s: &str) -> usize {
    s.len()
}
