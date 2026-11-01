//! src/typestate_pattern/mod.rs
use crate::print_title;

pub mod activity;
pub mod demo;

crate::tui_module! {
    description: "The typestate pattern",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        print_title("TypeState Pattern");

        demo::index(false);

        activity::index(false);
    }
}
