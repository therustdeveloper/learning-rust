mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Random Numbers");

        // Random Definition
        definition::master(false);
    }
}
