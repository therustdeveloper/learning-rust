//! src/object_oriented/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Object Oriented");

        definition::index(true);
    }
}
