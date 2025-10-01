//! src/threads/move_data_between_threads.rs
use std::sync::{Arc, Mutex};

pub fn index(show: bool) {
    if show {
        println!("---Move data between threads---");

        let my_number = Arc::new(Mutex::new(0));

        let cloned_1 = Arc::clone(&my_number);
        let cloned_2 = Arc::clone(&my_number);

        let thread1 = std::thread::spawn(move || {
            for _ in 0..10 {
                *cloned_1.lock().unwrap() += 1;
            }
        });

        let thread2 = std::thread::spawn(move || {
            for _ in 0..10 {
                *cloned_2.lock().unwrap() += 1;
            }
        });

        thread1.join().unwrap();
        thread2.join().unwrap();

        println!("Value is: {my_number:?}");
        println!("Exiting Program!");
    }
}
