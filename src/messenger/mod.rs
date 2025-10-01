//! src/messenger/mod.rs
pub mod definition;
pub mod mock_messenger;

pub fn index(show: bool) {
    if show {
        println!("\n-- Messenger");

        definition::index(true);
    }
}
