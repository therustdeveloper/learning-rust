//! src/random_numbers/definition.rs
use rand::RngExt;

pub fn index(show: bool) {
    if show {
        let mut rng = rand::rng();
        let secret_number = rng.random_range(1..=100);

        println!("The secret number is: {secret_number}");
    }
}
