//! src/new_type/definition.rs

use crate::print_title;

struct Hostname(String);

fn connect(hostname: Hostname) {
    println!("connected to {}", hostname.0);
}

pub fn index(show: bool) {
    if show {
        print_title("--- newtype Pattern");

        let ordinary_string = String::from("localhost");
        let host = Hostname(ordinary_string.clone());

        connect(host);
    }
}
