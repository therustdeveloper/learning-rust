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

enum Messages {
    // Message is in used
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Messages {
    fn call(&self) {
        match self {
            Messages::Quit => println!("Quit Message"),
            Messages::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Messages::Write(text) => println!("Write message: {}", text),
            Messages::ChangeColor(r, g, b) => {
                println!("Change color to red: {}, green: {}, blue: {}", r, g, b)
            }
        }
    }
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

        let mut msg = Messages::Move { x: 10, y: 10 };
        msg.call();
        msg = Messages::Quit;
        msg.call();
        msg = Messages::ChangeColor(255, 255, 255);
        msg.call();
        msg = Messages::Write(String::from("Internal Error"));
        msg.call();
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
