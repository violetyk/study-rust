fn main() {
    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "word".to_string());
    println!("Hello, {name}!");
}
