//! src/message_passing/mod.rs
mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Message Passing");

        definition::master(true);
    }
}
