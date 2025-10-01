//! src/hashmaps/hashmap_old_values.rs
use std::collections::HashMap;

pub fn index(show: bool) {
    if show {
        println!("--- HashMap Old Values");

        let mut book_hashmap = HashMap::new();
        let mut old_hashmap_values = Vec::new();

        let hashmap_entries = [
            (1, "L'Allemagne Moderne"),
            (1, "Le Petit Prince"),
            (1, "섀도우 오브 유어 스마일"),
            (1, "Eye of the World"),
        ];

        for (key, value) in hashmap_entries {
            if let Some(old_value) = book_hashmap.insert(key, value) {
                println!("Overwriting {old_value} with {value}!");
                old_hashmap_values.push(old_value);
            }
        }

        println!("All old values: {old_hashmap_values:?}");
    }
}
