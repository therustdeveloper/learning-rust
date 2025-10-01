//! src/variables/constants.rs
const TAX_RATE: f64 = 7.7;

pub fn index(show: bool) {
    if show {
        println!("--- Constants ---");

        println!("The value of TAX_RATE is {}", TAX_RATE);
    }
}
