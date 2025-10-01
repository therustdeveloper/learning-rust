//! src/errors/error_propagation.rs
use crate::print_title;
use std::fs;
use std::io;

pub fn index(show: bool) {
    if show {
        print_title("Return Errors");

        let f = read_username_from_file();

        let f = match f {
            Ok(username) => username,
            Err(e) => e.to_string(),
        };

        println!("In Error Propagation f is: {}", f);
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
