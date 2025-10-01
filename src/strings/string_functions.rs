//! src/strings/string_functions.rs
pub fn index(show: bool) {
    if show {
        println!("--- Working with string functions");

        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        println!("head: {}", head);
        println!("tail: {}", tail);

        case_demo(false);
    }
}

fn case_demo(show: bool) {
    if show {
        let sentence = "Rust is amaizing!";

        let transformed_sentence: String =
            sentence.chars().map(|c| c.to_ascii_uppercase()).collect();

        println!("Transformed: {}", transformed_sentence);
    }
}
