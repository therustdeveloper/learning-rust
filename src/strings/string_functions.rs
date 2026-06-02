//! src/strings/string_functions.rs
pub fn index(show: bool) {
    if show {
        println!("--- Working with string functions");

        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        println!("head: {}", head);
        println!("tail: {}", tail);

        case_demo(true);

        count_words_demo(true);

        common_string_functions(true);
    }
}

fn case_demo(show: bool) {
    if show {
        let sentence = "Rust is amazing!";

        let transformed_sentence: String =
            sentence.chars().map(|c| c.to_ascii_uppercase()).collect();

        println!("Transformed: {}", transformed_sentence);
    }
}

fn count_words_demo(show: bool) {
    if show {
        println!("--- Counting words in a sentence");

        let sentence = "Rust is a programming language. Rust is powerful.";
        let target_word = "Rust";
        let count = sentence.matches(target_word).count();

        println!("The word '{}' appears {} times", target_word, count);
    }
}

fn common_string_functions(show: bool) {
    if show {
        println!("--- Common string functions");

        let mut music_genres = "    Rock, Metal, Country, Rap    ";
        println!("{}", music_genres.trim());
        println!("{}", music_genres.trim_start());
        println!("{}", music_genres.trim_end());

        music_genres = music_genres.trim();
        println!("{}", music_genres.to_uppercase());
        println!("{}", music_genres.to_lowercase());

        println!("{}", music_genres.replace("a", "@"));

        let genres: Vec<&str> = music_genres.split(", ").collect();
        println!("{:?}", genres);
    }
}
