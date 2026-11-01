//! src/add_millimeters_to_meters/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Add millimeters to meters",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        // Array Definition
        definition::index(true);
    }
}
