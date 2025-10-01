//! src/iterators/result_option_ok.rs
pub fn index(show: bool) {
    if show {
        println!("---Result to Option with ok method");

        let user_input = [
            "8.9",
            "Nine point nine five",
            "8.0",
            "7.6",
            "eleventy-twelve",
        ];

        let successful_numbers = user_input
            .iter()
            .filter_map(|input| input.parse::<f32>().ok())
            .collect::<Vec<f32>>();

        println!("{:?}", successful_numbers);
    }
}
