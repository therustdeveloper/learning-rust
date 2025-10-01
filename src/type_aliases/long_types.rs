//! src/type_aliases/long_types.rs
type SkipFourTakeFive = std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>;

fn returns_some_chars(input: Vec<char>) -> SkipFourTakeFive {
    input.into_iter().skip(4).take(5)
}

pub fn index(show: bool) {
    if show {
        println!("---Long Types");

        let vector = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let skip_four_take_five = returns_some_chars(vector);

        println!("skip_four_take_file: {:?}", skip_four_take_five);
    }
}
