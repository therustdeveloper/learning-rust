//! src/closures/closures_inside_methods.rs
pub fn index(show: bool) {
    if show {
        println!("Closures Inside Methods");

        example1(false);
    }
}

fn example1(show: bool) {
    if show {
        println!("--- Closures for_each");

        (1..=3).for_each(|num| println!("{num}"));

        (1..=3).for_each(|num| {
            println!("Got a {num}!");
            if num % 2 == 0 {
                println!("It's even")
            } else {
                println!("It's odd")
            }
        });
    }
}
