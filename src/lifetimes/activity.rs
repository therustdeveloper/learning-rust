//! src/lifetimes/activity.rs
use crate::print_title;
use std::fs::File;
use std::io::{self, BufRead};

pub fn index(show: bool) {
    if show {
        print_title("Lifetimes Activity");

        process_file_custom(false);

        process_file_activity(true);
    }
}

fn process_file_activity(show: bool) {
    if show {
        print_title("Process File Activity");

        let file_path = "mock-data.csv";

        if let Ok(file) = File::open(file_path) {
            // create a buffered reader to read the file line by line
            let reader = io::BufReader::new(file);

            let data: Vec<_> = reader.lines().skip(1).collect();

            for item in data.iter() {
                match item {
                    Ok(line) => {
                        let fields: Vec<&str> = line.split(',').collect();
                        if fields.len() >= 2 {
                            let first_name = fields[1];
                            let title = fields[4];

                            println!("First Name: {}, Title: {}", first_name, title);
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        } else {
            println!("Error opening the file: {}", file_path);
        }
    }
}

fn process_file_custom(show: bool) {
    if show {
        print_title("Process File Custom");

        let file_path = "mock-data.csv";

        if let Ok(file) = File::open(file_path) {
            // create a buffered reader to read the file line by line
            let reader = io::BufReader::new(file);

            // iterate over the lines in the file
            for (index, line) in reader.lines().enumerate() {
                // skip the first line (header)
                if index == 0 {
                    continue;
                }

                // unwrap the line and split it by commas
                let line = line.unwrap();
                let fields: Vec<&str> = line.split(',').collect();

                // extract first_name and title
                if fields.len() >= 2 {
                    let first_name = fields[1];
                    let title = fields[4];

                    // print first_name and title
                    println!("First Name: {}, Title: {}", first_name, title);
                } else {
                    println!("Invalid CSV format for line {}", index);
                }
            }
        } else {
            println!("Error opening the file: {}", file_path);
        }
    }
}
