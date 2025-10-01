//! src/boxes/mod.rs
pub mod definition;
pub mod dereference;

pub fn index(show: bool) {
    if show {
        println!("\n-- Boxes");

        definition::index(true);
    }
}
