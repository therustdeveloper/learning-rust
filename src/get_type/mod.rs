//! src/get_type/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Get Type");

        definition::index(true);
    }
}
