//! src/rust_atomics_and_locks/chapter01/thread_closure.rs
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("---Thread Closure");

        let numbers = vec![1, 2, 3, 4, 5, 6, 7];

        thread::spawn(move || {
            for n in &numbers {
                println!("{n}");
            }
        })
        .join()
        .unwrap();
    }
}
