//! src/strings/mod.rs
pub mod borrowed_string;
pub mod concatenation;
pub mod definition;
pub mod iterate;
pub mod matches;
pub mod string;
pub mod string_functions;
pub mod string_is_encoded;
pub mod string_literals;
pub mod strs;
pub mod substrings;

crate::tui_module! {
    description: "String, &str and string functions",
    category: Basics,
    sections_only,
}
