//! src/loops/definition.rs
pub fn index(show: bool) {
    if show {
        println!("-- Loops");

        loop_print_numbers_1_to_4(false);

        loop_practice(false);
    }
}

pub fn loop_print_numbers_1_to_4(show: bool) {
    if show {
        println!("\n--- Loop to print numbers from 1 to 4");
        let mut i = 1;

        loop {
            println!("The value of i: {}", i);
            if i == 4 {
                break;
            }
            i += 1;
        }
    }
}

pub fn loop_practice(show: bool) {
    if show {
        let mut seven = 1;

        loop {
            if seven > 7 {
                break;
            }
            println!("The value of seven: {}", seven);
            seven += 1;
        }
    }
}
