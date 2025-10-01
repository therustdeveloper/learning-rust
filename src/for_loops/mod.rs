//! src/for_loops/mod.rs
use crate::print_title;
pub mod demo;

pub fn index(show: bool) {
    if show {
        print_title("For Loops");

        demo::definition(true);
    }
}
