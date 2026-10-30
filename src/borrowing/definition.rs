pub fn index(show: bool) {
    if show {
        println!("-- Borrowing Definition --");

        let s1 = String::from("Hello");
        let len = calculate_length(&s1);
        println!("Length of '{}': {}", s1, len);
    }
}

fn calculate_length(s: &str) -> usize {
    s.len()
}
