pub fn index(show: bool) {
    if show {
        let pizza_diameters = [8, 10, 12, 14];

        let pepperoni = String::from("Pepperoni");
        let mushroom = String::from("mushroom");
        let sausage = String::from("Sausage");

        // Rust moves the ownership of the variables to the pizza_toppings
        let pizza_toppings = [pepperoni, mushroom, sausage];

        // this code is not valid, because the ownership was moved
        //println!("pepperoni: {:?}", pepperoni);

        // this create a copy because integer implements the copy trait
        let value = pizza_diameters[2];
        println!("pizza diameter: {value}");

        // cannot move out of index, because does not implements the copy trait
        //let topping = pizza_toppings[2];

        // this creates a reference to the heap allocation
        let reference = &pizza_toppings[2];
        println!("reference value: {:?}", reference);
    }
}
