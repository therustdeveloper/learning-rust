//! src/strings/borrowed_string.rs
fn prints_str(my_str: &str) {
    println!("{my_str}");
}

pub fn index(show: bool) {
    if show {
        println!("Borrowed String");

        let my_string = String::from("I am a string");
        prints_str(&my_string);
    }
}
