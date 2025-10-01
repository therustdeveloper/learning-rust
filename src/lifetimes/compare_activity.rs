//! src/lifetimes/compare_activity.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("Compare Activity");

        let short = "hello";
        let long = "this is a long message";
        println!("{}", longest(short, long));
    }
}

fn longest<'a>(one: &'a str, two: &'a str) -> &'a str {
    if two > one {
        two
    } else {
        one
    }
}
