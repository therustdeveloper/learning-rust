//! src/advanced_match/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Advanced Match");

        definition::index(true);
    }
}
