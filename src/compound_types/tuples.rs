pub fn index(show: bool) {
    if show {
        println!("-- Tuples --");

        let values: (i32, f64, char) = (7, 77.0, '7');

        // Destructuring wit let
        let (x, y, z) = values;
        println!("Destructured Values: x = {}, y = {}, z = {}", x, y, z);

        // Direct Access
        let first_element = values.0;
        let second_element = values.1;
        let third_element = values.2;
        println!(
            "Direct Access: First element is: {}, Second element is: {}, Third element is: {}",
            first_element, second_element, third_element
        );
    }
}
