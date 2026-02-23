pub fn index(show: bool) {
    if show {
        let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
        println!("{:?}", string_vector);

        string_vector.push("four");
        println!("{:?}", string_vector);
    }
}
