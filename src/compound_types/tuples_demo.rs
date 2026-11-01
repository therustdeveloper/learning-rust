pub fn index(show: bool) {
    if show {
        println!("-- Tuples Demo --");

        let input1 = 7;
        let input2 = 77;

        // Call the function and destructure the returned tuple directly into variables.
        let (sum_result, product_result) = calculate_sum_and_product(input1, input2);

        println!("For {} and {}:", input1, input2);
        println!("  Sum: {}", sum_result);
        println!("  Product: {}", product_result);
    }
}

fn calculate_sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}
