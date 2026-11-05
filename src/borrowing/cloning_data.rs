pub fn index(show: bool) {
    if show {
        println!("-- Cloning Data --");

        let s1 = String::from("Hallo");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");
    }
}
