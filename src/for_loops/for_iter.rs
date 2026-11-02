pub fn index(show: bool) {
    if show {
        println!("-- for iter --");

        let a = [10, 20, 30, 40, 50, 60, 70];

        for element in a.iter() {
            println!("The value is: {}", element);
        }
    }
}
