//! src/draw/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Draw");

        definition::index(true);
    }
}
