pub fn index(show: bool) {
    if show {
        println!("-- String Matches");

        let sentence = "Rust is a programming language. Rust is powerful.";
        let target_word = "Rust";
        let count = sentence.matches(target_word).count();

        println!("The word '{}' appears {} times", target_word, count);
    }
}
