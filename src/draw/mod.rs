//! src/draw/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Draw");

        definition::master(true);
    }
}
