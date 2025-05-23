//! src/ownership/functions.rs
pub fn index(show: bool) {
    if show {
        println!("--- Moving a Value ---");
        let s = String::from("hallo");

        takes_ownership(s); // The compiler prevents us to pass the ownership

        //println!("the value of s is: {s}");

        let x = 7;
        makes_copy(x);
    }
}

// message receives the ownership of the s variable and s is not valid anymore in the master
// function
fn takes_ownership(message: String) {
    println!("message variable takes ownership of the string that s was the owner: {message}");
}

// it receives a copy of the x variable
fn makes_copy(some_integer: i32) {
    println!("some_integer value is: {some_integer}, some_integer is copied because integer implements copy");
}
