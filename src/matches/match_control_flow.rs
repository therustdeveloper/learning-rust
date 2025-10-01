//! src/matches/match_control_flow.rs
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn index(show: bool) {
    if show {
        println!("Match Control Flow");

        let coins = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];

        for coin in coins {
            println!("coin: {}", value_in_cents(coin));
        }
    }
}
