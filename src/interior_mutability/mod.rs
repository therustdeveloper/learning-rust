//! src/interior_mutability/mod.rs
mod definition;

pub fn master(show: bool) {
    if show {
        println!("Interior Mutability");

        definition::master(false);
    }
}
