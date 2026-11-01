//! src/destructuring_structs/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Destructuring structs with patterns",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Destructuring Structs");

        definition::index(true);
    }
}
