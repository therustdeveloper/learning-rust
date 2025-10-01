//! src/testing/activity.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Testing Activity");

        let clamp_result = clamp(77, 7, 777);
        println!("clamp result: {}", clamp_result);

        let div_result = div(77, 7);
        println!("div result: {:?}", div_result);

        let concat_result = concat("Metallica", "Rocks!");
        println!("concat_result: {}", concat_result);
    }
}

#[cfg(test)]
pub mod test {
    use crate::testing::activity::{clamp, concat, div};

    #[test]
    fn clamp_lower() {
        let result = clamp(7, 77, 777);
        let expected = 77;
        assert_eq!(result, expected, "should be 77");
    }

    #[test]
    fn clamp_upper() {
        let result = clamp(7777, 77, 777);
        let expected = 777;
        assert_eq!(result, expected, "should be 777");
    }

    #[test]
    fn clamp_n() {
        let result = clamp(77, 7, 777);
        let expected = 77;
        assert_eq!(result, expected, "should be 77");
    }

    #[test]
    fn check_div() {
        let result = div(7, 7);
        let expected = Some(1);
        assert_eq!(result, expected, "should be Some(1)");
    }

    #[test]
    fn check_div_zero() {
        let result = div(7, 0);
        let expected = None;
        assert_eq!(result, expected, "cannot divide by zero");
    }

    #[test]
    fn check_concat() {
        let result = concat("Metallica", "Rocks!");
        let expected = String::from("Metallica Rocks!");
        assert_eq!(result, expected, "should be Metallica Rocks!");
    }
}

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}
