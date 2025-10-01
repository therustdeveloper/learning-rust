pub fn index(show: bool) {
    if show {
        let values = [1, 2, 3, 4, 5, 6, 7];

        // this is a kind of dynamic slice
        let some_values = &values[..3];

        println!("some_values: {some_values:?}");

        // the type is exactly the same
        let other_slice = &values[4..];
        println!("other_slice: {other_slice:?}");

        let some_values = &other_slice;
        println!("some_values: {some_values:?}");
    }
}
