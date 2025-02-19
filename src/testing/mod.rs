//! src/testing/mod.rs
mod activity;
mod definition;
mod demo;

pub fn master(show: bool) {
    if show {
        println!("\n-- Tests");

        definition::master(false);
        demo::master(false);
        activity::master(true);
    }
}
