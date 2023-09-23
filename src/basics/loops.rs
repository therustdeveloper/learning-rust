pub fn master(show: bool) {
    if show {
        println!("\n--- Loops");

        for_loop_elements();

        countdown_loop_with_rev();
    }
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
