//! src/add_millimeters_to_meters/definition.rs
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn index(show: bool) {
    if show {
        println!("-- Implementing the Add trait on Millimeters to add Millimeters and Meters");

        let meters = Meters(7);
        println!("Meters is: {:?}", meters);

        let millimeters = Millimeters(7000);
        let mill_returned = millimeters.add(meters);

        println!("The millimeters returned are: {:?}", mill_returned);
    }
}
