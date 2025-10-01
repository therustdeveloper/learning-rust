//! src/exercises/ex03.rs
pub mod christmas_carol;
pub mod fah_to_cel;
pub mod fib;

pub fn index(show: bool) {
    if show {
        println!("\n--- Chapter 03");

        fah_to_cel::index();
        fib::index();
        christmas_carol::index();
    }
}
