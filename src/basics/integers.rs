//! src/basics/integers.rs
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn index(show: bool) {
    if show {
        println!("-- Integers");

        byte_literals();

        create_integer_values();
    }
}

fn byte_literals() {
    println!("-- Byte Literals");
    let x_ascii = b'x';
    println!("x in ASCII is: {}", x_ascii);
}

fn create_integer_values() {
    println!("-- Create integer values");
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
