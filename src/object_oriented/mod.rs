//! src/object_oriented/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Object-oriented programming in Rust",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Object Oriented");

        definition::index(true);
    }
}
