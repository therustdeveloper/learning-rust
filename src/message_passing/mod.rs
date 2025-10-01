//! src/message_passing/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Message Passing");

        definition::index(true);
    }
}
