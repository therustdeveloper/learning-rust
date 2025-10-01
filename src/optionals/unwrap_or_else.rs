//! src/optionals/unwrap_or_else.rs
pub fn index(show: bool) {
    if show {
        println!("Using unwrap_or_else to deal with None");

        let my_vec = [8, 9, 10];

        // unwrap_or is to pass a default value when receive None from Optional
        // unwrap_or_else is to give a default value, except that it passes on a closure that we
        // can use to write some more complex logic.
        let fourth = my_vec.get(3).unwrap_or_else(|| my_vec.get(2).unwrap_or(&0));

        println!("{fourth}");
    }
}
