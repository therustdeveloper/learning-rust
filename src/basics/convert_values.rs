//! src/basics/convert_values.rs
pub fn index(show: bool) {
    if show {
        let miles_away = 50;
        let miles_away_i8 = miles_away as i8;

        println!("miles_away_i8 value: {miles_away_i8}");

        let miles_away: f64 = 100.329032;
        let miles_away_f32 = miles_away as f32;
        let miles_away_int = miles_away as i32;

        println!("miles_away_f32: {miles_away_f32}");
        println!("miles away int value: {miles_away_int}");

        let guess: u32 = "77".parse().expect("Not a number!");
        println!("guess value: {guess}");
    }
}
