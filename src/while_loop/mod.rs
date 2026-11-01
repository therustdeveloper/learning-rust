//! src/while_loop/mod.rs
pub mod definition;
pub mod while_let;

crate::tui_module! {
    description: "while and while let loops",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- While Loops");

        definition::index(false);

        while_let::index(true);
    }
}
