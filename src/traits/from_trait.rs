//! src/traits/from_trait.rs
use crate::print_title;

#[allow(unused)]
#[derive(Debug)]
struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(data: String) -> Self {
        Uppercase(data.to_uppercase())
    }
}

impl From<&str> for Uppercase {
    fn from(data: &str) -> Self {
        Uppercase(data.to_uppercase())
    }
}

pub fn index(show: bool) {
    if show {
        print_title("From Trait");

        let upper = Uppercase::from("lowercase");
        println!("{:?}", upper);
        let upper: Uppercase = "lowercase".into();
        println!("{:?}", upper);
    }
}
