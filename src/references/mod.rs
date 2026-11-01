//! src/references/mod.rs
pub mod basic_reference;
pub mod definition;
pub mod mut_reference;
pub mod ref_of_references;

crate::tui_module! {
    description: "References and references of references",
    category: Basics,
    sections_only,
}
