//! src/threads/mutexes.rs

/*
A `Mutex` allows only one thread to access some data at any given time.
To access the data, a thread must first acquire the mutex's lock.
 */

use std::sync::{Arc, Mutex};
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("--- Mutexes");

        definition(false);
        mutex_example(false);
        mutex_try_lock(false);
        multithreading_with_mutex(false);
    }
}

fn definition(show: bool) {
    if show {
        println!("--- Definition");

        let my_mutex = Mutex::new(7);
        let mut mutex_charger = my_mutex.lock().unwrap();

        println!("{my_mutex:?}");
        println!("{mutex_charger:?}");

        *mutex_charger = 77;
        println!("{mutex_charger:?}");
    }
}

// ten threads, each incrementing a counter guarded by a Mutex<T>
fn mutex_example(show: bool) {
    if show {
        println!("--- Mutex Example");

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}

fn mutex_try_lock(show: bool) {
    if show {
        println!("--- Mutex try-lock");

        let my_mutex = Mutex::new(7);
        let mutex_changer = my_mutex.lock().unwrap();
        println!("{mutex_changer:?}");

        let other_mutex_changer = my_mutex.try_lock();

        if let Ok(value) = other_mutex_changer {
            println!("The MutexGuard has: {value}");
        } else {
            println!("Didn't get the lock");
        }
    }
}

fn multithreading_with_mutex(show: bool) {
    if show {
        let data = Arc::new(Mutex::new(0));
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let data = data.clone();
                thread::spawn(move || {
                    let mut data = data.lock().unwrap();
                    *data += 1;
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Final data value: {:?}", *data.lock().unwrap());
    }
}
