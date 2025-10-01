//! src/rust_atomics_and_locks/chapter01/thread_leaking.rs
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("Thread Leaking");

        let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

        thread::spawn(move || dbg!(x));
        thread::spawn(move || dbg!(x));
    }
}
