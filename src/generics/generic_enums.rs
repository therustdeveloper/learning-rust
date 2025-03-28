//! src/generics/generic_enums.rs
use crate::print_title;

pub fn master(show: bool) {
    if show {
        generic_enums(true);
    }
}

// Option<T> is a Generic Enum
fn generic_enums(show: bool) {
    if show {
        print_title("Generic Enums");

        let o1 = Some(7);
        let o2 = Some("seven");
        let o3 = Some(true);
        let o4 = Some(7.0);

        println!("o1: {:?}", o1);
        println!("o2: {:?}", o2);
        println!("o3: {:?}", o3);
        println!("o4: {:?}", o4);

        println!("safe_division of 4/2: {:?}", safe_division(4, 2));
    }
}

fn safe_division(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}
