mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Static Variable");

        definition::master(true);
    }
}
