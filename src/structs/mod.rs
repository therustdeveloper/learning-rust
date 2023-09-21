mod definition;
mod tuple_structs;
mod unit_struct;
mod human;
mod integrate_struct_enum_match;
mod structs_owned;
mod person;

pub fn master(show: bool) {
    if show {
        definition::master(false);

        tuple_structs::master(false);

        unit_struct::master(false);

        human::master(false);

        integrate_struct_enum_match::master(false);

        structs_owned::master(false);

        person::master(true);
    }
}