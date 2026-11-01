//! src/scalar_types/mod.rs
pub mod booleans;
pub mod characters;
pub mod floats;
pub mod integers;

crate::tui_module! {
    description: "Scalar Types",
    category: Basics,
    sections_only,
}
