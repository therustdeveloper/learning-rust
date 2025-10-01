//! src/while_loop/definition.rs
pub fn index(show: bool) {
    if show {
        println!("\n-- While Loops");

        conditional_loops_with_while();

        looping_through_collection_with_for();
    }
}

pub fn conditional_loops_with_while() {
    println!("\n--- Conditional Loops with While ---");

    let mut number = 7;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn looping_through_collection_with_for() {
    println!("\n--- Looping Through a Collection with For ---");

    let a = [10, 20, 30, 40, 50, 60, 77];
    let mut index = 0;

    while index < 7 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
