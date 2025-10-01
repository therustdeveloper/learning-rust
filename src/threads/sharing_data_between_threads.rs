//! src/threads/sharing_data_between_threads.rs
use std::sync::Arc;
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("---Sharing Data Between Threads");

        let shared_data = Arc::new(42);

        let handle = thread::spawn({
            let shared_data = shared_data.clone();

            move || {
                println!("{:?}", shared_data);
            }
        });

        match handle.join() {
            Ok(result) => {
                println!("Result: {:?}", result);
            }
            Err(_) => {
                println!("Error");
            }
        };

        println!("{:?}", ());
    }
}
