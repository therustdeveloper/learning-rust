mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Tests");

        definition::master(true);
    }
}
