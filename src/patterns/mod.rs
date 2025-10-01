//! src/patterns/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Patterns");

        definition::index(true);
    }
}
