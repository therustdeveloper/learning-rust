mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Operator Overloading");

        definition::master(true);
    }
}
