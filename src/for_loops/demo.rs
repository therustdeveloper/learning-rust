//! src/for_loops/demo.rs

pub fn index(show: bool) {
    if show {
        println!("For loop demo");

        let a = [10, 20, 30, 40, 50, 60, 70];

        for element in a {
            println!("The value is: {element}");
        }

        for_range();
    }
}

pub fn for_range() {
    for number in (1..8).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
