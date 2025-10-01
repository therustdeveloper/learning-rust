//! src/optionals/mod.rs
pub mod activity;
pub mod definition;
pub mod unwrap_or_else;

pub fn index(show: bool) {
    if show {
        println!("\n-- Optionals");

        definition::master(false);

        activity::master(false);

        unwrap_or_else::master(false);
    }
}
