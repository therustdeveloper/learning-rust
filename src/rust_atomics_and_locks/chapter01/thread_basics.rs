//! src/rust_atomics_and_locks/chapter01/thread_basics.rs
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("Thread Example");

        thread::spawn(f);
        thread::spawn(f);

        println!("Hello from the main thread.");
    }
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
