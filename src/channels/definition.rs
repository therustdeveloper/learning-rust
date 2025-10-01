//! src/channels/definition.rs
use std::sync::mpsc::channel;
use std::thread;

use num::range;

pub fn index(show: bool) {
    if show {
        println!("--- Channels Definition\n");

        let (sender, receiver) = channel();

        // spawn off an expensive computation
        thread::spawn(move || {
            {
                expensive_computation();
                sender.send(())
            }
            .unwrap()
        });

        // let's see what that answer was
        println!("{:?}", receiver.recv().unwrap());
    }
}

fn expensive_computation() {
    for n in range(0, 1_000_000_000) {
        println!("The value of n: {n}");
    }
}
