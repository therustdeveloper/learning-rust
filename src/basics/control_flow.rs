//! src/basics/control_flow.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Control Flow");

        if_expression(false);

        if_multiple_expressions(false);

        validate_season(false);

        conditional_statements(false);
    }
}

fn if_expression(show: bool) {
    if show {
        println!("--- If Expressions ---");

        let number = 7;

        if number < 77 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }
}

fn if_multiple_expressions(show: bool) {
    if show {
        println!("--- If with Multiple Expressions ---");
        let number = 7;

        if number % 7 == 0 {
            println!("number is divisible by 7");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 7, 3, or 2");
        }
    }
}

fn validate_season(show: bool) {
    if show {
        let season = "summer";

        if season == "summer" {
            println!("School's out!");
        } else if season == "winter" {
            println!("So cold!");
        } else if season == "fall" {
            println!("Leaves falling!");
        } else if season == "spring" {
            println!("Lots of rain!");
        }
    }
}

fn conditional_statements(show: bool) {
    if show {
        let num = 7;

        if num % 2 == 0 {
            println!("the number is even.");
        } else {
            println!("the number is odd.");
        }
    }
}
