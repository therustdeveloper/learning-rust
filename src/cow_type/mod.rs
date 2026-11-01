//! src/cow_type/mod.rs
use crate::print_title;
pub mod definition;

crate::tui_module! {
    description: "Clone-on-write with the Cow type",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        print_title("Using Cow");

        definition::index(false);
    }
}
