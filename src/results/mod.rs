//! src/results/mod.rs
pub mod activity;
pub mod definition;
pub mod question_mark_operator;

crate::tui_module! {
    description: "Working with Result<T, E> and the ? operator",
    category: Intermediate,
    sections_only,
}
