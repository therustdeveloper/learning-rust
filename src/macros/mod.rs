//! src/macros/mod.rs
pub mod create_vector;
pub mod definition;

crate::tui_module! {
    description: "Declarative macros",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        println!("--- Macros");

        definition::index(false);
        create_vector::index(false);
    }
}
