pub fn index(show: bool) {
    if show {
        let text = "50";
        let text_as_number = text.parse::<i32>();
        println!("Result Enum: {:?}", text_as_number);

        let text = "Alabama";
        let text_as_number = text.parse::<i32>();
        println!("Result Enum: {:?}", text_as_number);
    }
}
