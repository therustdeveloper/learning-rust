//! src/webserver/mod.rs
use crate::print_title;

pub mod web;

crate::tui_module! {
    description: "A multithreaded web server",
    category: System,
}

pub fn index(show: bool) {
    if show {
        print_title("Webserver");

        web::index(true);
    }
}
