trait Printable {
    fn print(&self);
}

// Number Implementation
struct Number {
    value: i32,
}

impl Printable for Number {
    fn print(&self) {
        println!("Number: {}", self.value);
    }
}

// Circle Implementation
struct Circle {
    radius: f64,
}

impl Printable for Circle {
    fn print(&self) {
        println!("Circule: {}", self.radius);
    }
}

// Rectangle Implementation
struct Rectangle {
    width: f64,
    height: f64,
}

impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle Width: {}", self.width);
        println!("Rectangle Height: {}", self.height);
    }
}

pub fn index(show: bool) {
    if show {
        println!("-- Printable Trait --");

        // Number implementation
        let number = Number { value: 7 };
        number.print();

        // Circle Implementation
        let circle = Circle { radius: 7.0 };
        circle.print();

        // Rectangle Implementation
        let rectangle = Rectangle {
            width: 7.0,
            height: 17.0,
        };
        rectangle.print();
    }
}
