//! src/optionals/mod.rs
pub mod activity;
pub mod definition;
pub mod unwrap_or_else;

crate::tui_module! {
    description: "Working with Option<T>",
    category: Basics,
    sections_only,
}
