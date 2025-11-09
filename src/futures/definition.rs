//! src/futures/definition.rs
use async_std;
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
use std::vec::Vec;
use std::{thread, time};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

pub fn index(show: bool) {
    if show {
        option1();

        using_futures_async();

        using_futures_join();

        using_futures_join();

        run_multiple_futures_at_once();
    }
}

fn option1() {
    // bad name for a function
    println!("\n-- Option");
    let now = time::Instant::now();
    let future_one = do_something(1);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    let outcome = block_on(future_one);
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);
}

fn using_futures_async() {
    println!("\n-- Using futures with async");
    let future_three = async {
        let outcome_one = do_something(2).await;
        let outcome_two = do_something(3).await;
        outcome_one + outcome_two
    };

    let future_outcome = block_on(future_three);
    println!("Here is the outcome with async: {:?}", future_outcome);
}

fn using_futures_join() {
    println!("\n-- Using futures with join");
    let future_four = async {
        let outcome_one = do_something(2);
        let outcome_two = do_something(3);
        let results = join!(outcome_one, outcome_two);
        results.0 + results.1
    };

    let now = time::Instant::now();
    let result = block_on(future_four);
    println!("time elapsed {:?}", now.elapsed());
    println!("here is the result: {:?}", result);
}

fn run_multiple_futures_at_once() {
    println!("\n-- Running multiple futures at once");
    let async_outcome = async {
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);

        futures_vec.push(future_four);
        futures_vec.push(future_five);

        let handles = futures_vec
            .into_iter()
            .map(async_std::task::spawn)
            .collect::<Vec<_>>();
        let results = join_all(handles).await;
        results.into_iter().sum::<i8>()
    };

    let now = time::Instant::now();
    let result = block_on(async_outcome);
    println!("{:?}", now);
    println!("{}", result);
    println!("time elapse")
}

/// Run futures examples with shutdown signal support for TUI mode
/// This version sends output through a channel and checks for shutdown signal
pub fn run_with_shutdown(tx: mpsc::Sender<String>, shutdown: Arc<AtomicBool>) {
    let send_msg = |msg: String| {
        let _ = tx.send(msg);
    };

    send_msg("Starting Futures Examples...".to_string());
    send_msg("".to_string());

    // Check shutdown before each operation
    if shutdown.load(Ordering::Relaxed) {
        send_msg("Stopped by user".to_string());
        return;
    }

    option1_with_channel(&tx, &shutdown);

    if shutdown.load(Ordering::Relaxed) {
        send_msg("Stopped by user".to_string());
        return;
    }

    using_futures_async_with_channel(&tx, &shutdown);

    if shutdown.load(Ordering::Relaxed) {
        send_msg("Stopped by user".to_string());
        return;
    }

    using_futures_join_with_channel(&tx, &shutdown);

    if shutdown.load(Ordering::Relaxed) {
        send_msg("Stopped by user".to_string());
        return;
    }

    run_multiple_futures_at_once_with_channel(&tx, &shutdown);

    send_msg("".to_string());
    send_msg("All futures examples completed!".to_string());
}

async fn do_something_with_channel(number: i8, tx: &mpsc::Sender<String>, shutdown: &Arc<AtomicBool>) -> i8 {
    let _ = tx.send(format!("number {} is running", number));

    // Sleep in smaller increments to check shutdown signal
    for _ in 0..20 {
        if shutdown.load(Ordering::Relaxed) {
            return 0;
        }
        thread::sleep(time::Duration::from_millis(100));
    }
    2
}

fn option1_with_channel(tx: &mpsc::Sender<String>, shutdown: &Arc<AtomicBool>) {
    let _ = tx.send("\n-- Option".to_string());
    let now = time::Instant::now();

    let tx_clone = tx.clone();
    let shutdown_clone = shutdown.clone();
    let future_one = async move {
        do_something_with_channel(1, &tx_clone, &shutdown_clone).await
    };

    // Sleep in smaller increments to allow shutdown
    for _ in 0..20 {
        if shutdown.load(Ordering::Relaxed) {
            let _ = tx.send("Stopped by user".to_string());
            return;
        }
        thread::sleep(time::Duration::from_millis(100));
    }

    let outcome = block_on(future_one);
    let _ = tx.send(format!("time elapsed {:?}", now.elapsed()));
    let _ = tx.send(format!("Here is the outcome: {}", outcome));
}

fn using_futures_async_with_channel(tx: &mpsc::Sender<String>, shutdown: &Arc<AtomicBool>) {
    let _ = tx.send("\n-- Using futures with async".to_string());

    let tx_clone = tx.clone();
    let shutdown_clone = shutdown.clone();
    let future_three = async move {
        if shutdown_clone.load(Ordering::Relaxed) {
            return 0;
        }
        let outcome_one = do_something_with_channel(2, &tx_clone, &shutdown_clone).await;

        if shutdown_clone.load(Ordering::Relaxed) {
            return 0;
        }
        let outcome_two = do_something_with_channel(3, &tx_clone, &shutdown_clone).await;
        outcome_one + outcome_two
    };

    let future_outcome = block_on(future_three);
    let _ = tx.send(format!("Here is the outcome with async: {:?}", future_outcome));
}

fn using_futures_join_with_channel(tx: &mpsc::Sender<String>, shutdown: &Arc<AtomicBool>) {
    let _ = tx.send("\n-- Using futures with join".to_string());

    let tx_clone = tx.clone();
    let shutdown_clone = shutdown.clone();
    let future_four = async move {
        let tx_1 = tx_clone.clone();
        let shutdown_1 = shutdown_clone.clone();
        let outcome_one = do_something_with_channel(2, &tx_1, &shutdown_1);

        let tx_2 = tx_clone.clone();
        let shutdown_2 = shutdown_clone.clone();
        let outcome_two = do_something_with_channel(3, &tx_2, &shutdown_2);

        let results = join!(outcome_one, outcome_two);
        results.0 + results.1
    };

    let now = time::Instant::now();
    let result = block_on(future_four);
    let _ = tx.send(format!("time elapsed {:?}", now.elapsed()));
    let _ = tx.send(format!("here is the result: {:?}", result));
}

fn run_multiple_futures_at_once_with_channel(tx: &mpsc::Sender<String>, shutdown: &Arc<AtomicBool>) {
    let _ = tx.send("\n-- Running multiple futures at once".to_string());

    let tx_clone = tx.clone();
    let shutdown_clone = shutdown.clone();
    let async_outcome = async move {
        let tx_4 = tx_clone.clone();
        let shutdown_4 = shutdown_clone.clone();
        let handle_four = async_std::task::spawn(async move {
            do_something_with_channel(4, &tx_4, &shutdown_4).await
        });

        let tx_5 = tx_clone.clone();
        let shutdown_5 = shutdown_clone.clone();
        let handle_five = async_std::task::spawn(async move {
            do_something_with_channel(5, &tx_5, &shutdown_5).await
        });

        let results = join_all(vec![handle_four, handle_five]).await;
        results.into_iter().sum::<i8>()
    };

    let now = time::Instant::now();
    let result = block_on(async_outcome);
    let _ = tx.send(format!("Start time: {:?}", now));
    let _ = tx.send(format!("Result: {}", result));
    let _ = tx.send("Time elapsed: see above".to_string());
}
