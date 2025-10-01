//! src/strings/string_literals.rs
pub fn index(show: bool) {
    if show {
        println!("String Literals");

        // String Literals
        // You make this when you write let my_str = ...
        // They last for the whole program because they are written directly into the binary.
        let my_str = "I am a &str";
        println!("my_str: {my_str:}");
    }
}
