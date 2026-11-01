//! src/patterns/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Patterns in if let, while let, for and functions",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Patterns");

        definition::index(true);
    }
}
