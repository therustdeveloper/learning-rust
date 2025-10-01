//! src/closures/demo.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Closures Demo");

        long_way();

        normal_way();
    }
}

fn long_way() {
    let add = |a: i32, b: i32| -> i32 { a + b };

    let sum = add(7, 7);

    println!("sum: {}", sum);
}

fn normal_way() {
    let add = |a, b| a + b;
    let sum = add(77, 77);
    println!("sum: {:?}", sum);
}
