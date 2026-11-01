//! src/boxes/mod.rs
pub mod definition;
pub mod dereference;

crate::tui_module! {
    description: "Heap allocation with Box<T> and dereferencing",
    category: Advanced,
    sections_only,
}
