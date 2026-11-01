//! src/ownership/mod.rs
pub mod borrowing;
pub mod dereference_operator;
pub mod functions;
pub mod mutable_borrowing;
pub mod mutable_references;
pub mod ownership_arrays;
pub mod references_borrowing;
pub mod return_multiple_values;
pub mod return_values_and_scope;
pub mod returning_ownership;
pub mod string_clone_heap;
pub mod string_type;

crate::tui_module! {
    description: "Ownership, borrowing and references",
    category: Basics,
    sections_only,
}
