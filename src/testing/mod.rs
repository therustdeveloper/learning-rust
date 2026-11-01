//! src/testing/mod.rs
pub mod activity;
pub mod definition;
pub mod demo;

crate::tui_module! {
    description: "Writing and running tests",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Tests");

        definition::index(false);
        demo::index(false);
        activity::index(true);
    }
}
