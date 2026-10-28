//! src/basics/num_operations.rs
pub fn index(show: bool) {
    if show {
        println!("--- Numeric Operations---");
        // addition
        let sum = 7 + 7;
        println!("value of sum is: {sum}");

        // subtraction
        let difference = 77.7 - 3.3;
        println!("value of difference is: {difference}");

        // multiplication
        let product = 7 * 77;
        println!("value of product is: {product}");

        // division
        let quotient = 77.7 / 17.7;
        println!("value of quotient is: {quotient}");
        let truncated = -7 / 7;
        println!("value of truncated is: {truncated}");

        // remainder or modulus
        let remainder = 77 % 7;
        println!("value of remainder is: {remainder}");

        // call sub function
        let seven = sub(10, 3);
        println!("value returned by sub function: {}", seven);

        // single operator
        let mut year = 2025;
        year += 1;
        year -= 1;
        year *= 1;
        year /= 2;

        println!("year: {year}");
    }
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}
