//! src/custom_smart_pointer/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Building a custom smart pointer",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Custom Smart Pointers");

        definition::index(true);
    }
}
