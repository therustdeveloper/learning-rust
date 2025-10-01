//! src/vectors/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Vectors");

        definition::index(true);
    }
}
