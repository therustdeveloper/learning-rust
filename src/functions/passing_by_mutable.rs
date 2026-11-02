fn takes_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn index(show: bool) {
    if show {
        println!("-- Passing by mutable --");

        let mut s = String::from("hello");
        takes_mutable_reference(&mut s);
        println!("s in main: {}", s);
    }
}
