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
        process_messages(msg);
        msg = Messages::Quit;
        process_messages(msg);
        msg = Messages::ChangeColor(255, 255, 255);
        process_messages(msg);
        msg = Messages::Write(String::from("Internal Error"));
        process_messages(msg);
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

fn process_messages(msg: Messages) {
    match msg {
        Messages::Quit => println!("Quit message"),
        Messages::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Messages::Write(text) => println!("Write message: {}", text),
        Messages::ChangeColor(r, g, b) => {
            println!("Change color to red: {}, green: {}, blue: {}", r, g, b)
        }
    }
}
