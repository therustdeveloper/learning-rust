mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Threads");

        definition::master(true);
    }
}
