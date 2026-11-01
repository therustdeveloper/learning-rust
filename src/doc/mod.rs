//! src/doc/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Documentation comments",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Doc");

        definition::index(true);
    }
}
