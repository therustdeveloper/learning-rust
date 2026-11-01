//! src/external_crates/mod.rs
pub mod activity;
pub mod demo;

crate::tui_module! {
    description: "Using external crates",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- External Crates");

        demo::index(false);
        activity::index(true);
    }
}
