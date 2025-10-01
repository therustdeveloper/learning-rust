pub fn index(show: bool) {
    if show {
        let text = "50";
        let text_as_number = text.parse::<i32>();
        println!("Result Enum: {:?}", text_as_number);

        let text = "Alabama";
        let text_as_number = text.parse::<i32>();
        println!("Result Enum: {:?}", text_as_number);

        let result = divide(7.7, 7.7);

        match result {
            Ok(calculation) => println!("calculation result: {}", calculation),
            Err(message) => println!("calculation error: {:?}", message),
        }
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}
