//! src/user_input/mod.rs
pub mod activity;
pub mod demo;

crate::tui_module! {
    description: "Reading user input",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n--- User Input");

        demo::index(false);
        activity::index(true);
    }
}
