pub fn index(show: bool) {
    if show {
        let mut count = 0;
        while count < 5 {
            println!("Current count: {}", count);
            count += 1;
        }
    }
}