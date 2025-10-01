//! src/interior_mutability/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("Interior Mutability");

        definition::index(false);
    }
}
