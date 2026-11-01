//! src/decisions/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Making decisions with if expressions",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Decisions");

        definition::index(true);
    }
}
