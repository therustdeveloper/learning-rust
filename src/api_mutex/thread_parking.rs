//! src/api_mutex/thread_parking.rs
use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn index(show: bool) {
    if show {
        println!("--- Thread parking");
        println!("Demonstrating thread parking with limited iterations (10 items)");

        let queue = Mutex::new(VecDeque::new());

        thread::scope(|s| {
            // Consuming thread - with finite loop
            let t = s.spawn(|| {
                let mut count = 0;
                loop {
                    let item = queue.lock().unwrap().pop_front();

                    if let Some(item) = item {
                        println!("Consumed item: {}", item);
                        count += 1;
                        if count >= 10 {
                            println!("Consumer finished after {} items", count);
                            break;
                        }
                    } else {
                        thread::park();
                        // Check if we should exit after being unparked
                        if queue.lock().unwrap().is_empty() && count >= 10 {
                            break;
                        }
                    }
                }
            });

            // Producing thread - limited to 10 items
            println!("Producing 10 items...");
            for i in 0..10 {
                queue.lock().unwrap().push_back(i);
                t.thread().unpark();
                thread::sleep(Duration::from_millis(100)); // Reduced sleep time for TUI
                println!("Produced item: {}", i);
            }

            // Unpark one last time to let consumer thread finish
            t.thread().unpark();
        });

        println!("Thread parking demonstration completed");
    }
}
