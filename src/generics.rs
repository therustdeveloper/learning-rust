pub mod definition;

pub fn master(show: bool) {
    if show {
        common::print_title("GENERICS");
        definition::master();
    }
}
