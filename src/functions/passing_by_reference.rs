pub fn index(show: bool) {
    if show {
        println!("-- Passing by reference --");

        let s1 = String::from("hallo");

        let len = calculate_length(&s1);

        println!("The lenght of '{}' is {}.", s1, len);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
