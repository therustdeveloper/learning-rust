//! src/iterators/skip.rs
pub fn index(show: bool) {
    if show {
        println!("---Iterator with skip");

        let ten_chars: Vec<char> = ('a'..).take(10).collect();
        let skip_then_ten_chars: Vec<char> = ('a'..).skip(1300).take(10).collect();

        println!("ten chars: {ten_chars:?}");
        println!("skip 1300 then ten chars: {skip_then_ten_chars:?}");
    }
}
