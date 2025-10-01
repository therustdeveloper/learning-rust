//! src/enums/hold_data.rs
enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

fn create_sky_state(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")),
        _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
    }
}

fn check_sky_state(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{description}"),
        ThingsInTheSky::Stars(description) => println!("{description}"),
    }
}

pub fn index(show: bool) {
    if show {
        println!("--- Enum holding data\n");

        let time = 8;
        let sky_state = create_sky_state(time);
        check_sky_state(&sky_state);
    }
}
