//! src/matches/while_let.rs
pub fn index(show: bool) {
    if show {
        println!("--- While let\n");

        let weather_vec = vec![
            vec!["Berlin", "cloudy", "5", "-7", "78"],
            vec!["Athens", "sunny", "not humid", "20", "10", "50"],
        ];

        for mut city in weather_vec {
            println!("For the city of {}:", city[0]);

            while let Some(information) = city.pop() {
                if let Ok(number) = information.parse::<i32>() {
                    println!("The number is: {number}");
                }
            }
        }
    }
}
