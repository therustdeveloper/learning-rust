//! src/slices/mod.rs
pub mod activity;
pub mod array_slices;
pub mod definition;
pub mod mutable_slice;
pub mod reference_array;

crate::tui_module! {
    description: "Array and string slices",
    category: Basics,
    sections_only,
}
