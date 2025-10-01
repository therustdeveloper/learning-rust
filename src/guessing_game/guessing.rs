//! src/guessing_game/guessing.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn index(show: bool) {
    if show {
        println!("Guess the number!");

        let secret_number = rand::rng().random_range(1..=100);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            // trim() is necessary because read_line includes the newline byte the buffer
            // if parse() is able to successfully turn the string into a number, it will return Ok
            // value that contains thre resultant number
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
