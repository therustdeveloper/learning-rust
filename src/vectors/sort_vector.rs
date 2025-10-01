pub fn index(show: bool) {
    if show {
        let mut ordered_vector = vec![1, 7, 6, 2, 3, 5, 4];
        println!("unordered vector: {:?}", ordered_vector);

        ordered_vector.sort();
        println!("ordered vector: {:?}", ordered_vector);
    }
}
