//! src/functions/mod.rs
pub mod blocks_in_functions;
pub mod definition;
pub mod func_return_option;
pub mod nested_functions;

crate::tui_module! {
    description: "Functions, nested functions and return values",
    category: Basics,
    sections_only,
}
