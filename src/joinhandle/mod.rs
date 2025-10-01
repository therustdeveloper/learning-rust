//! src/joinhandle/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Join Handle");

        definition::index(true);
    }
}
