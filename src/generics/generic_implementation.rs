//! src/generics/generic_implementation.rs
use std::fmt::Debug;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}");
}

pub fn index(show: bool) {
    if show {
        println!("--- Generic Implementation\n");

        let charlie = Animal {
            name: "Charlie".to_string(),
            age: 17,
        };

        let number = 55;

        println!("Name: {:?}", charlie.name);
        println!("Age: {:?}", charlie.age);

        print_item(charlie);
        print_item(number);
    }
}
