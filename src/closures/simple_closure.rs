//! src/closures/simple_closure.rs
pub fn index(show: bool) {
    if show {
        println!("Simple Closure");

        let my_closure = |x: i32| println!("{x}");

        my_closure(7);
        my_closure(7 + 7);

        simple_example(false);
    }
}

fn simple_example(show: bool) {
    if show {
        let multiply = |x: i32, y: i32| x * y;

        let product = multiply(7, 7);

        println!("Product: {}", product);
    }
}
