//! src/patterns/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Patterns");

        definition::master(true);
    }
}
