//! src/arrays/mod.rs
pub mod arrays_with_enums;
pub mod definition;
pub mod predictability;

pub fn index(show: bool) {
    if show {
        // Array Definition
        definition::index(true);

        // Array Predictability
        predictability::index(true);

        // Arrays with Enums
        arrays_with_enums::index(true);
    }
}
