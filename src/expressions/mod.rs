//! src/expressions/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Expressions and statements",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Expressions");

        definition::index(true);
    }
}
