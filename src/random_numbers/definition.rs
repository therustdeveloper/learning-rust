//! src/random_numbers/definition.rs
use rand::Rng;

pub fn index(show: bool) {
    if show {
        let secret_number = rand::rng().random_range(1..=100);

        println!("The secret number is: {secret_number}");
    }
}
