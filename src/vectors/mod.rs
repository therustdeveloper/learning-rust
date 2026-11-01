//! src/vectors/mod.rs
pub mod add_remove;
pub mod create_vector;
pub mod definition;
pub mod get_method;
pub mod reading_vector;
pub mod sort_vector;
pub mod vector_capacity;
pub mod vector_mutability;

crate::tui_module! {
    description: "Storing lists of values with Vec<T>",
    category: DataStructures,
    sections_only,
}
