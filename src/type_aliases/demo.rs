//! src/type_aliases/demo.rs
use crate::print_title;
use std::collections::HashMap;

struct Contact {
    name: String,
    phone: String,
}

type ContactName = String;
type ContactIndex = HashMap<ContactName, Contact>;
type Meters = i32;

pub fn index(show: bool) {
    if show {
        print_title("Type Aliases Demo");

        let contact_name: ContactName = String::from("Joe");

        let contact = Contact {
            name: contact_name,
            phone: String::from("3177777777"),
        };

        println!("contact name: {:?}", contact.name);
        println!("contact phone: {:?}", contact.phone);

        let mut contact_index: ContactIndex = HashMap::new();

        add_contact(&mut contact_index, contact);

        type_examples(false);
    }
}

fn add_contact(index: &mut ContactIndex, contact: Contact) {
    index.insert(contact.name.to_owned(), contact);
}

fn type_examples(show: bool) {
    if show {
        let mile_race_length: Meters = 1600;
        let two_mile_race_length: Meters = 3200;
        println!("A one mile race is {mile_race_length} meters long and two mile race length: {two_mile_race_length}");
    }
}
