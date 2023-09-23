#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

pub fn master(show: bool) {
    if show {
        let rect1 = Rectangle {
            width: 30,
            height: 70,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );

        if rect1.width() {
            println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }
    }
}
