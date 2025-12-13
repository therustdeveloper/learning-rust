pub fn index(show: bool) {
    if show {
        println!("--- Substrings ---");

        substrings_demo(true);
    }
}

pub fn substrings_demo(show: bool) {
    if show {
        println!("--- Substrings Demo ---");

        let sentence = "Rust is fun!";
        let first_word = &sentence[..4];

        println!("First word: {}", first_word);
    }
}
