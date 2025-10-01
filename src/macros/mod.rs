//! src/macros/mod.rs
pub mod create_vector;
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("--- Macros");

        definition::index(false);
        create_vector::index(false);
    }
}
