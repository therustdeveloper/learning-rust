//! src/tuples/mod.rs
pub mod definition;
pub mod single_tuple;

pub fn index(show: bool) {
    if show {
        println!("\n-- Tuples");

        definition::index(false);

        single_tuple::index(false);
    }
}
