pub fn index(show: bool) {
    if show {
        println!("-- Mutable Variables --");

        let mut x = 7;
        println!("The value of x is {}", x);
        x = 77;
        println!("The value of x is {}", x);
    }
}
