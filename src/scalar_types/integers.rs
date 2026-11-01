pub fn index(show: bool) {
    if show {
        println!("-- Integers --");

        let signed_int: i32 = -77;
        let unsigned_int: i32 = 77;
        println!(
            "Singed Integer: {}, Unsigned Integer: {}",
            signed_int, unsigned_int
        );
    }
}
