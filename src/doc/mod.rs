//! src/doc/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Doc");

        definition::master(true);
    }
}
