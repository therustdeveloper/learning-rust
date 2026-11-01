pub fn index(show: bool) {
    if show {
        println!("-- Shadowing Demo --");

        let x = 7;
        let x = x + 70;
        println!("The value of x is: {}", x);
    }
}
