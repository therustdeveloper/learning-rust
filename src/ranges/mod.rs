//! src/ranges/mod.rs
pub mod demo;

crate::tui_module! {
    description: "Working with ranges",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Ranges");

        demo::index(true);
    }
}
