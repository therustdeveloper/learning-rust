//! src/loops/mod.rs
pub mod definition;
pub mod returning_values_from_loops;
pub mod while_loops;

pub fn index(show: bool) {
    if show {
        println!("\n-- Lifetimes");

        definition::index(false);

        returning_values_from_loops::index(false);

        while_loops::index(false);
    }
}
