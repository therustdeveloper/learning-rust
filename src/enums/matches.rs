//! src/enums/matches.rs
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn index(show: bool) {
    if show {
        value_in_cents(Coin::Penny);
        value_in_cents(Coin::Nickel);
        value_in_cents(Coin::Dime);
        value_in_cents(Coin::Quarter(UsState::Alaska));
        value_in_cents(Coin::Quarter(UsState::Alabama));

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        println!("The value of five is: {:?}", five);
        println!("The value of six is: {:?}", six);
        println!("THe value of none is: {:?}", none);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
}
