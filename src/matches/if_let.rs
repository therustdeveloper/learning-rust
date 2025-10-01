//! src/matches/if_let.rs
pub fn index(show: bool) {
    if show {
        println!("\n-- if let else...");

        let maybe_user = Some("Bill");
        match maybe_user {
            Some(user) => println!("user: {:?}", user),
            None => println!("no user"),
        }

        // this is used when you don't care when there is no user
        if let Some(user) = maybe_user {
            println!("user: {:?}", user)
        }

        if_let_example();
        if_let_else_example();
    }
}

fn if_let_example() {
    let scores = [7, 77, 777, 7777];

    for index in 0..10 {
        if let Some(number) = scores.get(index) {
            println!("The number is: {number}")
        }
    }
}

fn if_let_else_example() {
    let scores = [7.7, 77.7, 777.7, 7777.7];

    for index in 0..10 {
        if let Some(number) = scores.get(index) {
            println!("The number is: {number}");
        }
        let Some(number) = scores.get(index) else {
            continue;
        };
        println!("The number is {number}")
    }
}
