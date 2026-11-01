//! src/scalar_types/mod.rs
use crate::print_title;
pub mod floats;
pub mod integers;

crate::tui_module! {
    description: "Scalar Types",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        print_title("Scalar Types");
    }
}
