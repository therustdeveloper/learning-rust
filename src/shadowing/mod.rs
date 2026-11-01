//! src/shadowing/mod.rs
use crate::print_title;
pub mod definition;

crate::tui_module! {
    description: "Variable shadowing",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        print_title("Shadowing");

        definition::index(false);
    }
}
