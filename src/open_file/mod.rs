use std::fs::File;
use std::io::{Error, Read};

crate::tui_module! {
    description: "Opening and reading a file",
    category: System,
}

pub fn index(show: bool) {
    if show {
        let filename = "mod.rs";
        match read_file_contents(filename) {
            Ok(contents) => println!("{:?}", contents),
            Err(err) => println!("{:?}", err),
        }
    }
}

fn read_file_contents(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}
