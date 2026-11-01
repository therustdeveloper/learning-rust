//! src/variables/mod.rs
pub mod constants;
pub mod copying_vars;
pub mod moving_vars;
pub mod mutable;
pub mod mutable_borrowing_vars;
pub mod vars;

crate::tui_module! {
    description: "Variables and mutability",
    category: Basics,
    sections_only,
}
