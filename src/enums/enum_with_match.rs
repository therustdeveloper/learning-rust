//! src/enums/enum_with_match.rs
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, EnumIter)]
enum Color {
    Yellow,
    Blue,
    Red,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Enum with Match");

        // Enum iteration
        for item in Direction::iter() {
            let direction = which_way(item);
            println!("Direction: {}", direction);
        }

        println!("\n-- Print Color");
        for color in Color::iter() {
            print_color(color);
        }
    }
}

fn which_way(go: Direction) -> &'static str {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

fn print_color(color: Color) {
    match color {
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("blue"),
        Color::Red => println!("red"),
    }
}
