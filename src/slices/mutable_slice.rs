pub fn index(show: bool) {
    if show {
        let mut values = [1, 2, 3, 4, 5, 6, 7];
        let mut_slice = &mut values[..4];

        println!("mut_slice: {:?}", mut_slice);
        mut_slice[0] = 77;

        println!("values: {:?}", values);
    }
}
