//! src/numbers/mod.rs

pub mod comparing_numbers;
pub mod complex_numbers;
pub mod try_into_method;

pub fn index(show: bool) {
    if show {
        println!("-- Numbers");

        intro_to_numbers(false);
        non_base2(false);
        comparing_numbers::comparing_different_types(false);
        try_into_method::convert_try_into(false);
        complex_numbers::create_complex_number(false);
    }
}

fn intro_to_numbers(show: bool) {
    if show {
        println!("--- Intro to Numbers:");

        let twenty = 20;
        let twenty_one: i32 = 21;
        let twenty_two = 22i32;

        let addition = twenty + twenty_one + twenty_two;
        println!(
            "{} + {} + {} = {}",
            twenty, twenty_one, twenty_two, addition
        );

        let one_million: i64 = 1_000_000;
        println!("{}", one_million.pow(2));

        let forty_twos = [42.0, 42f32, 42.0_f32];
        println!("{:02}", forty_twos[0]);
    }
}

fn non_base2(show: bool) {
    if show {
        println!("--- Non base2 Numbers:");

        let three = 0b11;
        let thirty = 0o36;
        let three_hundred = 0x12C;

        println!("base 10: {} {} {}", three, thirty, three_hundred);
        println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
        println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
        println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
    }
}
