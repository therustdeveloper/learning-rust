//! src/traits/simple_trait.rs
struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof woof!");
    }

    fn run(&self) {
        println!("The dog is running!");
    }
}

impl DogLike for Dog {}

impl DogLike for Parrot {
    fn run(&self) {
        println!("{} the parrot is running!", self.name);
    }
}

pub fn index(show: bool) {
    if show {
        println!("--- Simple Trait Example");

        let rover = Dog {
            name: "Rover".to_string(),
        };

        let brian = Parrot {
            name: "Brian".to_string(),
        };

        println!("{:?}", rover.name);
        rover.bark();
        rover.run();

        println!("{:?}", brian.name);
        brian.bark();
        brian.run();
    }
}
