//! src/new_type/demo.rs
use crate::print_title;

#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

impl NeverZero {
    pub fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("cannot be zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    let b = b.0;
    a / b
}

pub fn index(show: bool) {
    if show {
        print_title("New Type Pattern Demo");

        let dividend = 49;

        match NeverZero::new(0) {
            Ok(nz) => println!("{:?}", divide(dividend, nz)),
            Err(e) => println!("{:?}", e),
        }
    }
}
