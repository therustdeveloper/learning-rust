pub fn index(show: bool) {
    if show {
        println!("--- Predictability");

        let temperatures: [f64; 7] = [23.5, 25.1, 21.8, 20.7, 22.3, 24.9, 26.6];

        println!("temperatures: {:?}", temperatures);

        for temp in temperatures.iter() {
            println!("temp: {}", temp);
        }

        let fruits: [&str; 7] = [
            "apple",
            "mango",
            "pineapple",
            "strawberries",
            "watermelon",
            "orange",
            "banana",
        ];
        for fruit in fruits.iter() {
            println!("fruit: {}", fruit);
        }
    }
}
