pub fn index(show: bool) {
    if show {
        println!("-- Passing by value --");

        let x = 7;
        takes_value_copy(x);
        println!("Original value of x after function call: {}", x);
    }
}

fn takes_value_copy(mut some_integer: i32) {
    some_integer += 1;
    println!("Value inside function: {}", some_integer);
}
