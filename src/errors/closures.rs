//! src/errors/closures.rs
use crate::print_title;
use std::fs::File;
use std::io::ErrorKind;

pub fn index(show: bool) {
    if show {
        print_title("Errors using closures");

        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });

        println!("greeting_file is: {:?}", greeting_file);
    }
}
