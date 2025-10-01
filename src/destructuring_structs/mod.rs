//! src/destructuring_structs/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Destructuring Structs");

        definition::index(true);
    }
}
