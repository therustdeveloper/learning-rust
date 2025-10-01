//! src/user_input/mod.rs
pub mod activity;
pub mod demo;

pub fn index(show: bool) {
    if show {
        println!("\n--- User Input");

        demo::index(false);
        activity::index(true);
    }
}
