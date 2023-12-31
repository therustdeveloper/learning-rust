pub fn master(show: bool) {
    if show {
        println!("\n--- Loops");

        returning_values_from_loops();

        for_loop_elements();

        countdown_loop_with_rev();
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
}

fn countdown_loop_with_rev() {
    println!("--- Countdown Loop with Rev ---");

    for number in (1..7).rev() {
        println!("the value is: {number}!");
    }

    println!("LIFTOFF!!!");
}
