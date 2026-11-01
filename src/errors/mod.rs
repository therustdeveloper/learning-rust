//! src/errors/mod.rs
pub mod activity_custom_errors;
pub mod closures;
pub mod custom_errors;
pub mod demo_custom_error;
pub mod error_handling;
pub mod error_propagation;

crate::tui_module! {
    description: "Error handling, propagation and custom errors",
    category: Intermediate,
    sections_only,
}
