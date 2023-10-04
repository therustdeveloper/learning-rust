use crate::basics::constants;
use crate::print_title;

pub fn master(show: bool) {
    if show {
        print_title("Variables");

        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");

        println!("MAX_SPEED constant value: {}", constants::MAX_SPEED);
    }
}
