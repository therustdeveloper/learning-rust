//! src/joinhandle/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Waiting for threads with JoinHandle",
    category: Concurrency,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Join Handle");

        definition::index(true);
    }
}
