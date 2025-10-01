//! src/expressions/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Expressions");

        definition::index(true);
    }
}
