//! src/errors/mod.rs
use crate::print_title;

pub mod activity_custom_errors;
pub mod closures;
pub mod custom_errors;
pub mod demo_custom_error;
pub mod error_handling;
pub mod error_propagation;

crate::tui_module! {
    description: "Error handling, propagation and custom errors",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        print_title("Errors");

        closures::index(false);

        error_propagation::index(false);

        demo_custom_error::index(false);

        activity_custom_errors::index(false);

        error_handling::index(false);

        custom_errors::index(false);
    }
}
