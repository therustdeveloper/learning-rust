//! src/rust_atomics_and_locks/chapter01/thread_scoped.rs
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("Scoped Thread");

        let numbers = vec![1, 2, 3, 4, 5, 6, 7];

        thread::scope(|s| {
            s.spawn(|| {
                println!("length: {}", numbers.len());
            });
            s.spawn(|| {
                for n in &numbers {
                    println!("{n}");
                }
            });
        });
    }
}
