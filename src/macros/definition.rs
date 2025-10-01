//! src/macros/definition.rs
macro_rules! greetings {
    ($x: expr) => {
        println!("Hello, {}", $x);
    };
}

pub fn index(show: bool) {
    if show {
        println!("---Macros Definition");

        declarative_macros(true);
    }
}

fn declarative_macros(show: bool) {
    if show {
        println!("Declarative Macros");

        greetings!("Earthly"); // prints: "Hello, Earthly"
    }
}
