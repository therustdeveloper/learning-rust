//! src/hello_world/mod.rs
use crate::print_title;

crate::tui_module! {
    description: "Hello, world!",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        print_title("Hello World");
    }
}
