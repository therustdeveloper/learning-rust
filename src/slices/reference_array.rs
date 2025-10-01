pub fn index(show: bool) {
    if show {
        let values = [1, 2, 3, 4, 5, 6, 7];

        let reference = &values;
        println!("reference: {reference:?}");
        print_lenght(reference);

        let slice_of_three = &values[..3];
        println!("slice_of_three: {slice_of_three:?}");
        print_lenght(slice_of_three);
    }
}

fn print_lenght(reference: &[i32]) {
    println!("reference: {}", reference.len());
}
