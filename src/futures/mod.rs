//! src/futures/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Futures");

        definition::index(true);
    }
}
