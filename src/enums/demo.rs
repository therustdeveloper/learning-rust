//! src/enums/demo.rs
use crate::print_title;

/// Enums (short for enumerations) are a way to create compound data types in Rust.
/// They let us `enumerate` multiple possible variants of a type.
/// A struct has multiple fields. In contrast, an enum has multiple variants.
/// Enums enable you to build clearer types and decrease the number of illegal states
/// that your data can take.
enum Status {
    Queued,
    Running,
    Failed,
}

/// demo has three examples: [status, simple_demo, player_demo]
pub fn index(show: bool) {
    if show {
        print_title("Enum Demos");

        status(false);

        simple_demo(false);

        player_demo(true);
    }
}

fn status(show: bool) {
    if show {
        print_status(Status::Queued);
        print_status(Status::Running);
        print_status(Status::Failed);
    }
}

fn print_status(status: Status) {
    match status {
        Status::Queued => println!("queued"),
        Status::Running => println!("running"),
        Status::Failed => println!("failed"),
    }
}

/// Enum with unnamed fields.
#[allow(unused)]
#[derive(Debug)]
enum HealthBar1 {
    Alive(i8),
    Dead,
}

/// Enum with named fields.
#[derive(Debug)]
enum HealthBar2 {
    Alive { life: i8 },
    Dead,
}

fn simple_demo(show: bool) {
    if show {
        let is_alive_1 = HealthBar1::Alive(77);
        let is_dead_1 = HealthBar1::Dead;

        println!("Is alive 1: {:?}", is_alive_1);
        println!("Is dead 1: {:?}", is_dead_1);

        let is_alive_2 = HealthBar2::Alive { life: 77 };
        let is_dead_2 = HealthBar2::Dead;

        println!("Is alive 2: {:?}", is_alive_2);
        println!("Is dead 2: {:?}", is_dead_2);

        // To get the value of enum's named field.
        match is_alive_2 {
            HealthBar2::Alive { life } => {
                println!("Life: {}", life);
            }
            HealthBar2::Dead => {
                println!("The entity is dead.");
            }
        }
    }
}

#[derive(Debug)]
enum Player {
    Alive { life: i8 },
    KnockedOut { life: i8, turns_to_wait: i8 },
    Dead,
}

fn player_demo(show: bool) {
    if show {
        let dead_player = Player::Dead;
        print_player(dead_player);

        let alive_player = Player::Alive { life: 77 };
        print_player(alive_player);

        let knocked_out_player = Player::KnockedOut {
            life: 77,
            turns_to_wait: 7,
        };
        print_player(knocked_out_player);
    }
}

fn print_player(player: Player) {
    match player {
        Player::Alive { life } => println!("Player Alive: {}", life),
        Player::KnockedOut {
            life,
            turns_to_wait,
        } => println!(
            "Player Knocked Out. Life: {}, Turns to Wait: {}",
            life, turns_to_wait
        ),
        Player::Dead => println!("Player is dead!"),
    }
}
