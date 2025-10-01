//! src/futures/definition.rs
use async_std;
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
use std::vec::Vec;
use std::{thread, time};

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
