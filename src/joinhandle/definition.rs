//! src/joinhandle/definition.rs
use std::thread;
use std::time::Duration;

pub fn index(show: bool) {
    if show {
        println!("-- JoinHandle");
        definition();
    }
}

fn definition() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi humber {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
