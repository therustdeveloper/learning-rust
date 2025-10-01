//! src/generics/vehicle.rs
use crate::print_title;

trait Body {}

trait Color {}

#[derive(Debug)]
struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

impl<B: Body, C: Color> Vehicle<B, C> {
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

#[derive(Debug)]
struct Car;
impl Body for Car {}

#[derive(Debug)]
struct Truck;
impl Body for Truck {}

#[derive(Debug)]
struct Red;
impl Color for Red {}

#[derive(Debug)]
struct Blue;
impl Color for Blue {}

pub fn index(show: bool) {
    if show {
        print_title("Vehicle With Generic Structures");

        let red_truck = Vehicle::new(Truck, Red);
        println!("Car's Type: {:?}", red_truck.body);
        println!("Car's Color: {:?}", red_truck.color);

        let blue_car = Vehicle::new(Car, Blue);
        println!("Car's Type: {:?}", blue_car.body);
        println!("Car's Color: {:?}", blue_car.color);
    }
}
