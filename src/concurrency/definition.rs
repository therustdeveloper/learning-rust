//! src/concurrency/definition.rs
use std::thread;
use std::time::Duration;

pub fn index(show: bool) {
    if show {
        println!("Concurrency Definition");

        let secondary_thread = thread::spawn(|| {
            for i in 1..5 {
                // print i, then sleep for 2 milliseconds
                println!("Secondary Thread Prints {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            // print i, then sleep thread for 2 milliseconds
            println!("Main Thread Prints {}", i);
            thread::sleep(Duration::from_millis(1));
        }

        secondary_thread.join().unwrap();
    }
}
