pub fn index(show: bool) {
    if show {
        // this variable is a pointer to a memory on the heap
        let one: String = String::from("one");

        // this variable is also a pointer to a memory on the heap
        let two: String = one + "two";

        // in this moment the value of the variable one is move to variable two
        println!("value of two is: {}", two);

        // we cannot use the variable one
        // println!("value of one is: {}", one);
    }
}
