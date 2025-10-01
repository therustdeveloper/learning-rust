//! src/closures/vars_outside_closure.rs
pub fn index(show: bool) {
    if show {
        println!("Variables Outside of Closures");

        let number_one = 7;
        let number_two = 77;

        let my_closure = || println!("{}", number_one + number_two);
        my_closure();
    }
}
