use std::sync::mpsc::channel;
use std::thread;

use num::range;

pub fn master(show: bool) {
    if show {
        println!("--- Channels Definition\n");

        let (sender, receiver) = channel();

        // spawn off an expensive computation
        thread::spawn(move || sender.send(expensive_computation()).unwrap());

        // let's see what that answer was
        println!("{:?}", receiver.recv().unwrap());
    }
}

fn expensive_computation() {
    for n in range(0, 1_000_000_000) {
        println!("The value of n: {n}");
    }
}
