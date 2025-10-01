//! src/errors/error_handling.rs
use std::fs::File;

pub fn index(show: bool) {
    if show {
        println!("--- Error Handling");

        let result = File::open("non-existing-file.txt");

        match result {
            Ok(file) => println!("File opened successfully: {:?}", file),
            Err(err) => println!("Error opening file: {:?}", err),
        }
    }
}
