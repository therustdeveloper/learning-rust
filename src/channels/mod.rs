//! src/channels/mod.rs
use crate::print_title;
pub mod definition;

crate::tui_module! {
    description: "Communicating between threads with channels",
    category: Concurrency,
}

pub fn index(show: bool) {
    if show {
        print_title("Channels");

        // channels definition
        definition::index(false);
    }
}
