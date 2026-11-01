//! src/random_numbers/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Generating random numbers",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Random Numbers");

        // Random Definition
        definition::index(false);
    }
}
