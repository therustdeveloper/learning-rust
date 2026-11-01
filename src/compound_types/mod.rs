//! src/compound_types/mod.rs
use crate::print_title;
pub mod arrays;
pub mod tuples;
pub mod tuples_demo;

crate::tui_module! {
    description: "Compound Types",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        print_title("Compound Types");
    }
}
