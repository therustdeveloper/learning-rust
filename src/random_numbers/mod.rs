//! src/random_numbers/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Random Numbers");

        // Random Definition
        definition::index(false);
    }
}
