//! src/unsafe_trait/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Unsafe Trait");

        definition::index(true);
    }
}
