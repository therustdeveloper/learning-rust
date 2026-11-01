//! src/exercises/mod.rs
pub mod ex03;
pub mod ex08;

crate::tui_module! {
    description: "Chapter exercises",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Exercises");

        ex03::index(false);

        ex08::index(false);
    }
}
