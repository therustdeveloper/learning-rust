//! src/hashmaps/hashmap_entry_method.rs
use std::collections::HashMap;

pub fn index(show: bool) {
    if show {
        println!("--- HashMap Entry Method\n");

        let book_collection = vec![
            "L'Allemagne Moderne",
            "Le Petit Prince",
            "Eye of the World",
            "Eye of the World",
        ];

        let mut book_hashmap = HashMap::new();

        for book in book_collection {
            let return_value = book_hashmap.entry(book).or_insert(0);
            *return_value += 1;
        }

        for (book, number) in book_hashmap {
            println!("{book}, {number}");
        }
    }
}
