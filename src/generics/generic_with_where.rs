//! src/generics/generic_with_where.rs
use std::cmp::PartialOrd;
use std::fmt::Display;

pub fn index(show: bool) {
    if show {
        println!("--- Generic with where\n");

        compare_and_display("Listen up!", 9, 8);
    }
}

fn compare_and_display<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,              // This parameter must implement Display
    U: Display + PartialOrd, // This parameter must implement Display and PartialOrd
{
    println!(
        "{statement}! Is {num_1} greater than {num_2}? {}",
        num_1 > num_2
    );
}
