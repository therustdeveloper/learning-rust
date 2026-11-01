//! src/message_passing/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Message passing between threads",
    category: Concurrency,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Message Passing");

        definition::index(true);
    }
}
