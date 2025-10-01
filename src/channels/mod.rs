//! src/channels/mod.rs
use crate::print_title;
pub mod definition;

pub fn index(show: bool) {
    if show {
        print_title("Channels");

        // channels definition
        definition::index(false);
    }
}
