pub fn get_int() -> u64 {
    let mut input = String::new();
    if let Ok(_size) = std::io::stdin().read_line(&mut input) {}
    input.trim().parse().unwrap()
}
