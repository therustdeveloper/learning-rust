//! src/generics/generic_add.rs

use crate::print_title;
use std::ops::Add;
use std::time::Duration;

pub fn index(show: bool) {
    if show {
        print_title("Generic Add");

        let floats = add(1.2, 3.4);
        let ints = add(10, 20);
        let durations = add(Duration::new(5, 0), Duration::new(10, 0));

        println!("floats: {}", floats);
        println!("ints: {}", ints);
        println!("durations: {:?}", durations);
    }
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
