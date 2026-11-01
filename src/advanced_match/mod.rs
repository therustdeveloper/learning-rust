//! src/advanced_match/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Advanced match expressions with enums and vectors",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Advanced Match");

        definition::index(true);
    }
}
