//! src/get_type/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Getting the type of a value",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Get Type");

        definition::index(true);
    }
}
