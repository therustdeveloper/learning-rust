//! src/traits/with_generics.rs
use crate::print_title;
use std::io::Write;

pub fn index(show: bool) {
    if show {
        print_title("Traits with Generics");

        let mut v: Vec<u8> = Vec::from([1, 2, 3]);
        println!("{:?}", say_hello(&mut v));
    }
}

fn say_hello<T: Write>(out: &mut T) -> std::io::Result<()> {
    out.write_all(b"Hello World!")?;
    out.flush()
}
