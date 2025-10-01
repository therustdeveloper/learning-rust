//! src/exercises/ex08/pig_latin.rs
pub fn index() {
    println!("-- Convert strings to pig latin ---");

    let word = String::from("orange");

    println!("Result: {}", pigyfy(&word));
}

fn pigify_one(word: &str) -> String {
    let mut chars = word.chars();

    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}

fn folder(mut current: String, next: String) -> String {
    if !current.is_empty() {
        current.push(' ');
    }
    current.push_str(&next);
    current
}

fn pigyfy(text: &str) -> String {
    text.split_whitespace()
        .map(pigify_one)
        .fold(String::new(), folder)
}
