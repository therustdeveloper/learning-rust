//! src/decisions/mod.rs
mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Decisions");

        definition::master(true);
    }
}
