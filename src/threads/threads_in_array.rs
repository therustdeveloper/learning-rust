//! src/threads/threads_in_array.rs
use std::sync::{Arc, Mutex};

pub fn index(show: bool) {
    if show {
        println!("---Threads in array--");

        let my_number = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let my_number_clone = Arc::clone(&my_number);
            let handle = std::thread::spawn(move || {
                for _ in 0..10 {
                    *my_number_clone.lock().unwrap() += 1;
                }
            });
            handles.push(handle);
        }

        handles.into_iter().for_each(|h| h.join().unwrap());
        println!("{my_number:?}")
    }
}
