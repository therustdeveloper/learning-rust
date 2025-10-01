//! src/threads/preventing_race_condition.rs
use crate::print_title;
use std::thread;

pub fn index(show: bool) {
    if show {
        print_title("Preventing Race Condition");

        let data = 100;

        let thread1 = thread::spawn(move || {
            println!("data value on first spawn: {}", data);
        });
        let thread2 = thread::spawn(move || {
            println!("data value on second spawn: {}", data);
        });

        println!("Data value: {}", data);
        thread1.join().unwrap();
        thread2.join().unwrap();
    }
}
