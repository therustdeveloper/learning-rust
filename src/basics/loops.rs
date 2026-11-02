//! src/basics/loops.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Loops");

        returning_values_from_loops();

        for_loop_elements();

        countdown_loop_with_rev();

        loop_labels_multiple_loops();

        while_loop();
    }
}

fn returning_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}

fn for_loop_elements() {
    println!("--- For Loop Elements ---");

    let a = [10, 20, 30, 40, 50, 60, 77];

    for element in a {
        println!("the value is: {element}");
    }

    // with iter()
    for element in a.iter() {
        println!("the value is: {element}");
    }
}

fn countdown_loop_with_rev() {
    println!("--- Countdown Loop with Rev ---");

    for number in (1..7).rev() {
        println!("the value is: {number}!");
    }

    println!("LIFTOFF!!!");
}

pub fn loop_labels_multiple_loops() {
    println!("\n--- Loop Labels Multiple Loops ---");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn while_loop() {
    println!("\n--- While Loop ---");

    let mut number = 7;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
