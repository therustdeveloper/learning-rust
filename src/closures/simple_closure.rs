//! src/closures/simple_closure.rs
pub fn index(show: bool) {
    if show {
        println!("Simple Closure");

        let my_closure = |x: i32| println!("{x}");

        my_closure(7);
        my_closure(7 + 7);
    }
}
