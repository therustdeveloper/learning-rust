pub fn index(show: bool) {
    if show {
        let x = 5;
        let y = 7;

        fn inner_function(a: i32, b: i32) -> i32 {
            a * b
        }

        let result = inner_function(x, y);
        println!("Product: {}", result);
    }
}
