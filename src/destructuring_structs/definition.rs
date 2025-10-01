//! src/destructuring_structs/definition.rs
pub fn index(show: bool) {
    if show {
        definition();
        idiomatic_alternative();
        destructuring_matching_literal();
        destructuring_enums();
        destructuring_nested_structs_and_enums();
        destructuring_structs_tuples();
        ignoring_multiple_parts_of_tuple();
        matching_first_and_last_in_tuple();
        bindings();
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn definition() {
    println!("-- Basic Destructuring Structs");
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("Value of a: {a}");
    println!("Value of b: {b}");
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn idiomatic_alternative() {
    println!("-- Idiomatic Alternative");
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn destructuring_matching_literal() {
    println!("Destructuring Matching Literal");
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructuring_enums() {
    println!("-- Destructuring Enums");
    let _msg1 = Message1::Quit;
    let _msg2 = Message1::Move { x: 7, y: 7 };
    let _msg3 = Message1::Write(String::from("Hallo"));
    let msg4 = Message1::ChangeColor(0, 160, 255);

    match msg4 {
        Message1::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message1::Move { x, y } => {
            println!("Move in the x dir {x}, in the y dir {y}");
        }
        Message1::Write(text) => {
            println!("The message: {text}");
        }
        Message1::ChangeColor(r, g, b) => {
            println!("Change color to ree {r}, green {g}, and blue {b}")
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructuring_nested_structs_and_enums() {
    println!("-- Destructuring Nested Structs and Enums");
    let _msg1 = Message2::Quit;
    let _msg2 = Message2::Move { x: 7, y: 7 };
    let _msg3 = Message2::Write(String::from("Hallo"));
    let color1 = Color::Hsv(0, 160, 255);
    let _color2 = Color::Rgb(0, 160, 255);
    let msg4 = Message2::ChangeColor(color1);

    match msg4 {
        Message2::Move { x, y } => {
            println!("Move in the x dir {x}, in the y dir {y}");
        }
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}")
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color hue {h}, saturation {s}, value {v}")
        }
        Message2::Write(s) => {
            println!("Message to write: {}", s)
        }
        _ => (),
    }
}

fn destructuring_structs_tuples() {
    println!("-- Destructuring Structs and Tuples");

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Feet value: {feet}");
    println!("Inches value: {inches}");
    println!("Point x value: {x}");
    println!("Point y value: {y}");
}

fn ignoring_multiple_parts_of_tuple() {
    println!("-- Ignoring Multiple Parts of a Tuple");

    let numbers = (2, 4, 8, 16, 32);

    /*match numbers {
        (first, _, third, _, fifth) =>  {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }*/

    let (first, _, third, _, fifth) = numbers;
    {
        println!("Some numbers: {first}, {third}, {fifth}");
    }
}

fn matching_first_and_last_in_tuple() {
    println!("-- Matching only the first and last values in a tuple");
    let numbers = (2, 4, 8, 16, 32);

    let (first, .., last) = numbers;
    {
        println!("First and last: {first}, {last}");
    }

    /*match numbers {
        (first, .., last) => {
            println!("First and last: {first}, {last}");
        }
    }*/
}

enum Message3 {
    Hello { id: i32 },
}

fn bindings() {
    println!("-- @ Bindings");

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Some other id: {id}"),
    }
}
