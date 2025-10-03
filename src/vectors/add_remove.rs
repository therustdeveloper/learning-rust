pub fn index(show: bool) {
    if show {
        let mut pizza_diameters = vec![8, 10, 12, 14];
        pizza_diameters.push(16);
        pizza_diameters.push(18);
        pizza_diameters.insert(0, 4);
        pizza_diameters.insert(1, 6);

        let removed_pizza_diameter = pizza_diameters.pop();
        match removed_pizza_diameter {
            Some(value) => println!("value removed: {}", value),
            None => println!("error removing element"),
        }

        println!("pizza diameters: {:?}", pizza_diameters);
    }
}
