//! src/patterns/definition.rs
pub fn index(show: bool) {
    if show {
        println!("-- Patterns");

        if_let();
        while_let();
        for_loop();
        run_print_coordinates();
    }
}

fn if_let() {
    println!("-- If let");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    println!("-- While let");
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn for_loop() {
    println!("-- For loop");
    // let v = vec!['a', 'b', 'c'];

    for (index, value) in ['a', 'b', 'c'].iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

fn run_print_coordinates() {
    println!("-- Run print coordinates");
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
