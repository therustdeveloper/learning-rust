//! src/operator_overloading/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Operator Overloading");

        definition::index(true);
    }
}
