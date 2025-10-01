//! src/threads/scoped_threads.rs
use std::sync::Mutex;
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("---Scoped Threads---");

        let my_number = Mutex::new(0);

        thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..10 {
                    *my_number.lock().unwrap() += 1;
                }
            });

            s.spawn(|| {
                for _ in 0..10 {
                    *my_number.lock().unwrap() += 1;
                }
            });
        })
    }
}
