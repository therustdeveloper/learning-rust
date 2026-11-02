enum Command {
    Start,
    Stop,
    Move(i32, i32),
}

pub fn index(show: bool) {
    if show {
        println!("-- Enum Commands --");

        process_command(Command::Start);
        process_command(Command::Move(10, 20));
        process_command(Command::Stop);
    }
}

fn process_command(command: Command) {
    match command {
        Command::Start => println!("Starting..."),
        Command::Stop => println!("Stopping..."),
        Command::Move(x, y) => println!("Moving to coordinates: x = {}, y = {}", x, y),
    }
}
