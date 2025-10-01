//! src/mutable_reference/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Mutable Reference");

        definition::index(false);
    }
}
