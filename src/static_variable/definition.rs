//! src/static_variable/definition.rs
pub fn index(show: bool) {
    if show {
        println!("-- Accessing or Modifying a Mutable Static Variable");

        definition();
    }
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn definition() {
    println!("Value of static variable is: {HELLO_WORLD}");

    add_to_count(7);

    /*unsafe {
        println!("COUNTER: {COUNTER}");
    }*/
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
