//! src/arrays/mod.rs
pub mod arrays_with_enums;
pub mod definition;
pub mod predictability;

crate::tui_module! {
    description: "Fundamental Rust Arrays",
    category: Basics,
    sections: [
        definition => "Definition",
        predictability => "Predictability",
        arrays_with_enums => "Arrays with Enums",
    ],
}

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
