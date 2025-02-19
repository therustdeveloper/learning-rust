//! src/closures/process_closure_thread.rs
use std::thread::JoinHandle;
use std::{thread, time};

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

pub fn master(show: bool) {
    if show {
        let now = time::Instant::now();
        let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
        let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
        let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));
        let result_one = thread_one.join();
        let result_two = thread_two.join();
        let result_three = thread_three.join();
        println!("time elapsed {:?}", now.elapsed());
        println!(
            "result {}",
            result_one.unwrap() + result_two.unwrap() + result_three.unwrap()
        );
    }
}
