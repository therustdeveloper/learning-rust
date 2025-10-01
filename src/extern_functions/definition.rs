//! src/extern_functions/definition.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Using Extern Functions to Call External Code");

        definition();
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn definition() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
