//! src/iterators/scan_hash_table.rs
use std::collections::HashMap;

pub fn index(show: bool) {
    if show {
        println!("--- Scan a Hash Table");

        let mut populations = HashMap::new();
        populations.insert("Portland", 583_776);
        populations.insert("Fossil", 449);
        populations.insert("Greenhorn", 2);
        populations.insert("Boring", 7_762);
        populations.insert("Dalles", 15_340);

        assert_eq!(
            populations.iter().max_by_key(|&(_name, pop)| pop),
            Some((&"Portland", &583_776))
        );
        assert_eq!(
            populations.iter().min_by_key(|&(_name, pop)| pop),
            Some((&"Greenhorn", &2))
        );
    }
}
