//! src/collections/mod.rs
pub mod vectors;
pub mod hashmaps;

pub fn index(show: bool) {
    if show {
        //common::print_title("COLLECTIONS");
        println!("\n-- Collections");

        // Vectors
        vectors::index(false);

        // HashMaps
        hashmaps::index(false);
    }
}
