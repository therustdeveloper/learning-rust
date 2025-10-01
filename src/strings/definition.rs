//! src/strings/definition.rs
pub fn index(show: bool) {
    if show {
        println!("--- String Definition ---");
        let data = "This is a first string literal";

        // to_string method is available on any type that implements the Display trait
        let s1 = data.to_string();

        println!("{}", s1);

        // the method also works on a literal directly
        // this creates the following in the stack:
        // Reference to the Heap
        // Length: 31
        // Capacity: 40
        let s2 = "This is a second string literal".to_string();

        println!("{}", s2);

        // we can also use the following function to create String from a string literal
        let s3 = String::from("This is a third string literal");

        println!("{}", s3);

        // push a value into a String
        pushing_to_string();

        // push a single character
        push_single_character();
    }
} // drop(s3), drop(s2) and drop(s1) are called automatically
  // owners that have heap memory

fn pushing_to_string() {
    println!("--- Pushing to String ---");
    let mut s = String::from("Mutable string");

    println!("initial value of s String: {}", s);

    s.push_str(", this value was pushed to s variable");

    println!("After push: {}", s);
}

fn push_single_character() {
    println!("--- Push a Single Character ---");
    let mut s = String::from("lo");

    println!("value of s: {}", s);

    s.push('l');

    println!("value of s after push: {}", s);
}
