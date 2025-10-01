//! src/unsafe_code/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Unsafe Code");

        definition::index(true);
    }
}
