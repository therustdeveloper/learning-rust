pub fn index(show: bool) {
    if show {
        // replaced with the argument
        println!("{} days", 31);

        // positional arguments
        println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

        // named arguments
        // commented because my neovim ide is complaining
        /*println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );*/

        // invoke different formatted
        println!("Base 10:                 {}", 69420);
        println!("Base 2 (binary):       {:b}", 69420);
        println!("Base 8 (octal):        {:o}", 69420);
        println!("Base 16 (hexadecimal): {:x}", 69420);

        // right justify
        println!("{number:>7}", number = 7);

        // extra zeros
        println!("{number:0>7}", number = 7);

        // left-adjust
        println!("{number:0<7}", number = 7);

        // named arguments in the format specifier
        println!("{number:0>width$}", number = 7, width = 7);

        // capture arguments from a surrounding variable
        let number: f64 = 1.0;
        let width: usize = 7;
        println!("{number:>width$}");
    }
}
