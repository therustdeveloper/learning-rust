pub fn index(show: bool) {
    if show {
        let pepperoni = String::from("Pepperoni");
        let mushroom = String::from("mushroom");
        let sausage = String::from("Sausage");

        // Rust moves the ownership of the variables to the pizza_toppings
        let pizza_toppings = [pepperoni, mushroom, sausage];

        let option = pizza_toppings.get(2);
        match option {
            Some(pizza_topping) => println!("pizza topping: {:?}", pizza_topping),
            None => println!("No topping selected"),
        }
    }
}
