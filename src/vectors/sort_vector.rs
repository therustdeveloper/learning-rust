pub fn index(show: bool) {
    if show {
        let mut ordered_vector = vec![1, 7, 6, 2, 3, 5, 4];
        println!("unordered vector: {:?}", ordered_vector);

        ordered_vector.sort();
        println!("ordered vector: {:?}", ordered_vector);

        ordered_vector.reverse();
        println!("reverse vector: {:?}", ordered_vector);

        let find_element = ordered_vector.contains(&77);

        match find_element {
            true => println!("value found"),
            false => println!("value not found!"),
        }
    }
}
