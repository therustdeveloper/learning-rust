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

        test_error_check(false);
    }
}

fn test_error_check(show: bool) {
    if show {
        println!("--- Test Error Checking");

        println!("{:?}", error_check(false));
        println!("{:?}", error_check(false).is_err());
        println!("{:?}", error_check(true));
        println!("{:?}", error_check(true).is_err());
    }
}

fn error_check(check: bool) -> Result<i8, &'static str> {
    if check {
        Err("this is an error")
    } else {
        Ok(1)
    }
}