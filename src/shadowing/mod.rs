//! src/shadowing/mod.rs
use crate::print_title;
pub mod definition;

pub fn index(show: bool) {
    if show {
        print_title("Shadowing");

        definition::index(false);
    }
}
