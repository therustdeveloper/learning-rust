//! src/tuples/single_tuple.rs
pub fn index(show: bool) {
    if show {
        println!("--- Single tuple");

        let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);

        println!(
            "Inside the tuple is: \n\
            First Item: {:?}\n\
        Second Item: {:?}\n\
        Third Item: {:?}\n\
        Fourth Item: {:?}\n\
        Fifth Item: {:?}\n\
        Sixth Item: {:?}\n",
            random_tuple.0,
            random_tuple.1,
            random_tuple.2,
            random_tuple.3,
            random_tuple.4,
            random_tuple.5
        );
    }
}
