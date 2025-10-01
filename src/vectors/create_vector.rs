pub fn index(show: bool) {
    if show {
        let pizza_diameters: Vec<i32> = Vec::new();
        println!("pizza diameters: {:?}", pizza_diameters);

        // If I know the data to store in the vector
        let mut pizza_diameters = vec![8, 10, 12, 14];
        println!("pizza diameters: {:?}", pizza_diameters);

        // remove all the elements of a vector
        pizza_diameters.clear();
        println!("pizza diameters: {:?}", pizza_diameters);

        let pastas: Vec<&str> = Vec::new();
        println!("pastas: {:?}", pastas);

        // This is when we have the information to populate the vector
        let pastas = vec!["Rigatoni", "Angel hair", "Fettucine"];
        println!("pastas: {:?}", pastas);
    }
}
