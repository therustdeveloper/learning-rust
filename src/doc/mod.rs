//! src/doc/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Doc");

        definition::index(true);
    }
}
