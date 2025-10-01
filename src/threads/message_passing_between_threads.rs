//! src/threads/message_passing_between_threads.rs
use std::sync::mpsc; // multiple producer, single consumer
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("---Message Passing Between Threads");

        let (tx, rx) = mpsc::channel();

        let _ = thread::spawn(move || match rx.recv() {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        });

        let data = 42;
        match tx.send(data) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        };
    }
}
