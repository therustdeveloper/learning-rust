//! src/decisions/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Decisions");

        definition::index(true);
    }
}
