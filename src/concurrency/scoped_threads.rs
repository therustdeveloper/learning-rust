//! src/concurrency/scoped_threads.rs
use std::sync::Mutex;
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("Scoped Threads");

        let shared_state = Mutex::new(0);

        thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..10 {
                    *shared_state.lock().unwrap() += 1;
                }
            });
            s.spawn(|| {
                for _ in 0..10 {
                    *shared_state.lock().unwrap() += 1;
                }
            });
        });

        println!("Result: {:?}", shared_state);
    }
}
