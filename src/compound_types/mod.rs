//! src/compound_types/mod.rs
use crate::print_title;
pub mod tuples;

crate::tui_module! {
    description: "Compound Types",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        print_title("Compound Types");
    }
}
