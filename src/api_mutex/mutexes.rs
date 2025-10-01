//! src/api_mutex/mutexes.rs
use std::sync::Mutex;
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("--- Mutex in Practice");

        let n = Mutex::new(0);

        thread::scope(|s| {
            for _ in 0..10 {
                s.spawn(|| {
                    let mut guard = n.lock().unwrap();
                    for _ in 0..100 {
                        *guard += 1;
                    }
                });
            }
        });

        assert_eq!(n.into_inner().unwrap(), 1000);
    }
}
