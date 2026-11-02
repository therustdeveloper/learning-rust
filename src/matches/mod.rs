//! src/matches/mod.rs
pub mod catch_all;
pub mod definition;
pub mod if_let;
pub mod match_control_flow;
pub mod match_guard;
pub mod match_vector;
pub mod matching_literals;
pub mod matching_variables;
pub mod multiple_placeholders;
pub mod pattern_match;
pub mod placeholder;
pub mod while_let;

crate::tui_module! {
    description: "Pattern matching with match, if let and while let",
    category: Basics,
    sections_only,
}
