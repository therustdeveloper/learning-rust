fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn master(show: bool) {
    if show {
        println!("function definition");

        let x = add(1, 1);
        let y = add(2, 2);
        let z = add(3, 3);

        println!("x = {}, y = {}, z = {}", x, y, z);
    }
}