//! src/futures/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Async programming with futures",
    category: Concurrency,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Futures");

        definition::index(true);
    }
}
