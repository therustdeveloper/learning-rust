//! src/ranges/mod.rs
pub mod demo;

pub fn index(show: bool) {
    if show {
        println!("\n-- Ranges");

        demo::index(true);
    }
}
