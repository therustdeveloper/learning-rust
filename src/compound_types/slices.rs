pub fn index(show: bool) {
    if show {
        println!("-- Slices --");

        let array = [10, 20, 30, 40, 50, 60, 70];

        // Create a slice that references elements from index 1 up to (but not including) index 3
        // The type of `slice` is `&[i32]
        let slice = &array[1..3];
        println!("Original Array: {:?}", array);
        println!("Slice (a view into the array): {:?}", slice);
    }
}
