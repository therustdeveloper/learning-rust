//! src/message_passing/definition.rs
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::thread;
use std::time::Duration;

pub fn index(show: bool) {
    if show {
        println!("-- Message Passing to Transfer Data Between Threads");
        definition();
    }
}

fn definition() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

/// Run message passing examples with shutdown signal support for TUI mode
/// This version sends output through a channel and checks for shutdown signal
pub fn run_with_shutdown(tx: mpsc::Sender<String>, shutdown: Arc<AtomicBool>) {
    let send_msg = |msg: String| {
        let _ = tx.send(msg);
    };

    send_msg("-- Message Passing to Transfer Data Between Threads".to_string());
    send_msg("".to_string());
    send_msg("Starting two threads that send messages...".to_string());
    send_msg("".to_string());

    // Create internal channel for message passing demo
    let (demo_tx, demo_rx) = mpsc::channel();

    let demo_tx1 = demo_tx.clone();
    let shutdown1 = shutdown.clone();

    // First thread
    let thread1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // Check shutdown before sending
            if shutdown1.load(Ordering::Relaxed) {
                return;
            }

            demo_tx1.send(val).unwrap();

            // Sleep in smaller increments to check shutdown more frequently
            for _ in 0..10 {
                if shutdown1.load(Ordering::Relaxed) {
                    return;
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    });

    let shutdown2 = shutdown.clone();

    // Second thread
    let thread2 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            // Check shutdown before sending
            if shutdown2.load(Ordering::Relaxed) {
                return;
            }

            demo_tx.send(val).unwrap();

            // Sleep in smaller increments to check shutdown more frequently
            for _ in 0..10 {
                if shutdown2.load(Ordering::Relaxed) {
                    return;
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    });

    // Receive messages and forward to output channel
    let mut messages_received = 0;
    let expected_messages = 8;

    loop {
        // Check shutdown signal
        if shutdown.load(Ordering::Relaxed) {
            send_msg("".to_string());
            send_msg("Shutdown signal received".to_string());
            break;
        }

        // Try to receive with timeout so we can check shutdown periodically
        match demo_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(received) => {
                messages_received += 1;
                send_msg(format!("Got: {}", received));

                // Check if all messages received
                if messages_received >= expected_messages {
                    send_msg("".to_string());
                    send_msg("All messages received!".to_string());
                    break;
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Continue loop to check shutdown
                continue;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // Both senders dropped, we're done
                send_msg("".to_string());
                send_msg(format!(
                    "Message passing completed. Received {} messages.",
                    messages_received
                ));
                break;
            }
        }
    }

    // Wait for threads to finish
    let _ = thread1.join();
    let _ = thread2.join();

    send_msg("".to_string());
    send_msg("Message passing demo finished.".to_string());
}
