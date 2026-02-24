pub fn index(show: bool) {
    if show {
        let mut one: i8 = 5;
        print(&mut one);
        println!("In main the value is: {}", one);
    }
}

fn print(value: &mut i8) {
    *value += 1;
    println!("In print function the value is: {}", value);
}
