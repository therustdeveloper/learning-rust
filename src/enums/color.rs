#[derive(Debug)]
enum Kolor {
    // A variant that holds a tuple of three 8-bit unsigned integers
    Rgb(u8, u8, u8),

    // A varian that holds a single String
    Named(String),
}

pub fn index(show: bool) {
    if show {
        println!("-- Defining Enums --");

        let red = Kolor::Rgb(255, 0, 0);
        let custom_color = Kolor::Named(String::from("Forest Green"));

        println!("An RGB color: {:?}", red);
        println!("A named color: {:?}", custom_color);

        // Reading the fields back out with match
        describe(&red);
        describe(&custom_color);
    }
}

fn describe(color: &Kolor) {
    match color {
        Kolor::Rgb(r, g, b) => println!("RGB values: r={r}, g={g}, b={b}"),
        Kolor::Named(name) => println!("Color name: {name}"),
    }
}
