//! src/vectors/mod.rs
pub mod create_vector;
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Vectors");

        definition::index(true);

        create_vector::index(false);
    }
}
