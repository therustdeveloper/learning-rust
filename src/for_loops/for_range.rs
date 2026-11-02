pub fn index(show: bool) {
    if show {
        println!("-- for -range --");

        for number in 1..5 {
            println!("Exclusive range value: {}", number);
        }

        for number in 1..=5 {
            println!("Inclusive range value: {}", number);
        }
    }
}
