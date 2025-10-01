//! src/new_type/activity.rs
use crate::print_title;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[allow(unused)]
#[derive(Debug, EnumIter)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[allow(unused)]
#[derive(Debug)]
struct ShirtColor(Color);

impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[allow(unused)]
#[derive(Debug)]
struct ShoesColor(Color);

impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[allow(unused)]
#[derive(Debug)]
struct PantsColor(Color);

impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shirt_color(color: ShirtColor) {
    println!("shirt color = {:?}", color);
}

fn print_shoe_color(color: ShoesColor) {
    println!("shoe color = {:?}", color);
}

fn print_pant_color(color: PantsColor) {
    println!("pant color = {:?}", color);
}

pub fn index(show: bool) {
    if show {
        print_title("New Type Patter Activity");

        for color in Color::iter() {
            let shirt_color = ShirtColor::new(color);
            print_shirt_color(shirt_color);
        }

        let shoe_color = ShoesColor::new(Color::Blue);
        print_shoe_color(shoe_color);

        let pant_color = PantsColor::new(Color::Gray);
        print_pant_color(pant_color);
    }
}
