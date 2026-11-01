//! src/draw/mod.rs
pub mod definition;

crate::tui_module! {
    description: "The Draw trait and trait objects",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Draw");

        definition::index(true);
    }
}
