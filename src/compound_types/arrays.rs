pub fn index(show: bool) {
    if show {
        println!("-- Arrays --");

        let array: [i32; 3] = [1, 2, 3];
        println!("Array Values: {} {} {}", array[0], array[1], array[2]);
    }
}
