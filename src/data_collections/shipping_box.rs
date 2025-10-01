//! src/data_collections/shipping_box.rs
enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Color: brown"),
            Color::Red => println!("Color: red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    name: String,
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(name: String, weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            name,
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        println!("\nName: {:?}", self.name);
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Shipping Box Implementation");

        let small_dimensions = Dimensions {
            width: 1.0,
            height: 2.0,
            depth: 3.0,
        };

        let small_box =
            ShippingBox::new("Small Box".to_string(), 5.0, Color::Red, small_dimensions);
        small_box.print();

        let brown_dimensions = Dimensions {
            width: 2.0,
            height: 3.0,
            depth: 4.0,
        };

        let brown_box =
            ShippingBox::new("Brown Box".to_string(), 7.0, Color::Brown, brown_dimensions);
        brown_box.print();
    }
}
