//! src/mutable_reference/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Mutable references",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Mutable Reference");

        definition::index(false);
    }
}
