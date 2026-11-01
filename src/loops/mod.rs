//! src/loops/mod.rs
pub mod definition;
pub mod for_loops;
pub mod returning_values_from_loops;
pub mod while_loops;

crate::tui_module! {
    description: "loop, while and for loops",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Lifetimes");

        definition::index(false);

        returning_values_from_loops::index(false);

        while_loops::index(false);

        for_loops::index(false);
    }
}
