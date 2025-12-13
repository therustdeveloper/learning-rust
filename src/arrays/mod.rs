//! src/arrays/mod.rs
pub mod definition;
pub mod predictability;

pub fn index(show: bool) {
    if show {
        // Array Definition
        definition::index(true);

        // Array Predictability
        predictability::index(true);
    }
}
