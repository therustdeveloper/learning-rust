//! src/type_aliases/mod.rs
use crate::print_title;
pub mod demo;
pub mod long_types;

crate::tui_module! {
    description: "Type aliases",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        print_title("Type Aliases");

        demo::index(false);

        long_types::index(false);
    }
}
