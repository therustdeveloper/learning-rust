pub fn index(show: bool) {
    if show {
        println!("-- Strings --");

        let mut s = String::from("Hallo");
        s.push_str(", Rust");
        println!("{}", s);
    }
}
