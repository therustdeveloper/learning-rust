//! src/threads/sending_multiple_values.rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn index(show: bool) {
    if show {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Go: {received}");
        }
    }
}
