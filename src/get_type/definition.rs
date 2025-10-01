//! src/get_type/definition.rs
pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

#[derive(Debug)]
struct Human {
    name: String,
    age: i8,
}

pub fn index(show: bool) {
    if show {
        println!("-- Get Variable Type");

        let my_number = 7.77;
        let x = 7;
        let is_cool = true;
        let human = Human {
            name: String::from("Will"),
            age: 48,
        };
        let z: i8 = 7;

        println!("-- Human Struct Information");
        println!("{}", human.name);
        println!("{}", human.age);

        println!("\n-- Print Variables Types");
        println!("{}", my_number.type_name());
        println!("{}", x.type_name());
        println!("{}", is_cool.type_name());
        println!("{}", human.type_name());
        println!("{}", z.type_name());
    }
}
