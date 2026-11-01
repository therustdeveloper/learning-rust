//! src/post/mod.rs
pub mod definition;

crate::tui_module! {
    description: "The state pattern: a blog post workflow",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Post");

        definition::index(true);
    }
}
