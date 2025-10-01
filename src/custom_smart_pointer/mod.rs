//! src/custom_smart_pointer/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Custom Smart Pointers");

        definition::index(true);
    }
}
