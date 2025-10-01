//! src/rust_atomics_and_locks/chapter01/thread_join.rs
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("Join Thread");

        let t1 = thread::spawn(f);
        let t2 = thread::spawn(f);

        println!("Hello from the main thread.");

        t1.join().unwrap();
        t2.join().unwrap();
    }
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
