use crate::print_title;

mod activity;
mod conveyor_belt;
mod definition;
mod demo;
mod generic_structures;
mod vehicle;

pub fn master(show: bool) {
    if show {
        print_title("Generics");

        definition::master(false);

        demo::master(false);

        activity::master(false);

        generic_structures::master(false);

        conveyor_belt::master(false);

        vehicle::master(false);
    }
}
