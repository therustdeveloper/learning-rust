//! src/threads/definition.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn index(show: bool) {
    if show {
        definition(false);
        spawned_thread(false);
        sharing_mutex(false);
        working_with_threads(false);
        scoped_threads(false);
    }
}

fn definition(show: bool) {
    if show {
        println!("-- Creating a New Thread with Spawn");

        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        // this is the main thread
        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
}

fn spawned_thread(show: bool) {
    if show {
        println!("-- Spawned Thread");

        let v = vec![1, 2, 3, 4, 5, 6, 7];

        // using the move keyword to force a closure to take ownership of the values it use
        // // using the move keyword to force a closure to take ownership of the values it usess
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        // this join the thread with the main thread
        // when we call the join method on it, will wait for its thread to finish
        handle.join().unwrap();
    }
}

fn sharing_mutex(show: bool) {
    if show {
        println!("-- Sharing Mutex");

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}

fn working_with_threads(show: bool) {
    if show {
        println!("-- Working with threads");

        let t1 = thread::spawn(f);
        let t2 = thread::spawn(f);

        println!("Hello from main thread.");

        t1.join().unwrap();
        t2.join().unwrap();
    }
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}

fn scoped_threads(show: bool) {
    if show {
        println!("Scoped Threads");

        let numbers = vec![1, 2, 3];

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
