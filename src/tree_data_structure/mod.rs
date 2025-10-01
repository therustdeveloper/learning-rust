//! src/tree_data_structure/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Tree Data Structure");

        definition::index(true);
    }
}
