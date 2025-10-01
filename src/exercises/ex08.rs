//! src/exercises/ex08.rs
pub mod departments;
pub mod pig_latin;
pub mod vector;

pub fn index(show: bool) {
    if show {
        println!("\n--- Chapter 08");

        vector::index();

        pig_latin::index();

        departments::index();
    }
}
