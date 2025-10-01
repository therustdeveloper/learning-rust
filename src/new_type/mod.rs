//! src/new_type/mod.rs
use crate::print_title;
pub mod activity;
pub mod definition;
pub mod demo;

pub fn index(show: bool) {
    if show {
        print_title("New Type Pattern");

        demo::index(false);

        activity::index(false);

        definition::index(false);
    }
}
