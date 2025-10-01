//! src/webserver/mod.rs
use crate::print_title;

pub mod web;

pub fn index(show: bool) {
    if show {
        print_title("Webserver");

        web::index(true);
    }
}
