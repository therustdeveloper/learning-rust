//! src/generics/mod.rs
use crate::print_title;

pub mod activity;
pub mod conveyor_belt;
pub mod definition;
pub mod demo;
pub mod generic_add;
pub mod generic_enums;
pub mod generic_functions;
pub mod generic_implementation;
pub mod generic_structs;
pub mod generic_structures;
pub mod generic_with_where;
pub mod introduction;
pub mod multiple_generics;
pub mod question_operator;
pub mod vehicle;

pub fn index(show: bool) {
    if show {
        print_title("Generics");

        definition::index(false);

        demo::index(false);

        activity::index(false);

        generic_structures::index(false);

        conveyor_belt::index(false);

        vehicle::index(false);

        generic_functions::master(false);

        generic_enums::index(false);

        generic_structs::master(false);

        question_operator::master(false);

        generic_add::master(false);

        generic_implementation::master(false);

        generic_with_where::master(false);

        introduction::index(false);

        multiple_generics::index(false);
    }
}
