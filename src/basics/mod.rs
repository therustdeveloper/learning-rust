//! src/basics/mod.rs
pub mod arrays;
pub mod booleans;
pub mod borrowing;
pub mod branches;
pub mod characters;
pub mod clone;
pub mod compounds;
pub mod constants;
pub mod control_flow;
pub mod convert_values;
pub mod floats;
pub mod functions;
pub mod if_let;
pub mod integers;
pub mod loops;
pub mod match_statements;
pub mod numeric_operations;
pub mod recursion;
pub mod scalars;
pub mod scopes;
pub mod shadowing;
pub mod slices;
pub mod type_casting;
pub mod underscores;
pub mod unicode;
pub mod variables;

crate::tui_module! {
    description: "Fundamental Rust concepts and syntax",
    category: Basics,
    sections_only,
}
