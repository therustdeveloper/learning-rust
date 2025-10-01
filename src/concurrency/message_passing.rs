//! src/concurrency/message_passing.rs
use std::sync::mpsc;
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("Message Passing");

        // create a channel
        let (sender, receiver) = mpsc::channel();

        // create a new thread, create a value, send the value using the channel to main thread
        thread::spawn(move || {
            let val =
                String::from("I was created in the child thread, will be sent to main thread");
            sender.send(val).unwrap();
        });

        // receive and print the message from the child thread
        let received = receiver.recv().unwrap();
        println!(
            "I have received this message from the child thread: {}",
            received
        );
    }
}
