//! src/generics/conveyor_belt.rs
use crate::print_title;

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T: Convey> {
    pub items: Vec<T>,
}

impl<T: Convey> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 7.0,
            height: 6.0,
            depth: 5.0,
            weight: 4.0,
            part_number: "WMR977".to_owned(),
        }
    }
}

impl Convey for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}

pub fn index(show: bool) {
    if show {
        print_title("Generic Structures Demo");

        let mut belt: ConveyorBelt<CarPart> = ConveyorBelt { items: vec![] };
        belt.add(CarPart::default());

        for i in belt.items {
            println!("Width: {:?}", i.dimensions().width);
            println!("Height: {:?}", i.dimensions().height);
            println!("Depth: {:?}", i.dimensions().depth);
            println!("Part Number: {:?}", i.part_number);
            println!("Weight: {:?}", i.weight());
        }
    }
}
