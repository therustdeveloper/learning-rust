//! src/results/activity.rs
#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}

fn print_adult(adult: &Result<Adult, &str>) {
    println!("adult: {:?}", adult);
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Result Activity");

        let child = Adult::new(15, "Anita");
        let adult = Adult::new(21, "Sanjay");

        print_adult(&child);
        print_adult(&adult);

        match child {
            Ok(inner_child) => println!("{} is {} years old.", inner_child.name, inner_child.age),
            Err(e) => println!("error: {}", e),
        }

        match adult {
            Ok(inner_adult) => println!("{} is {} years old.", inner_adult.name, inner_adult.age),
            Err(e) => println!("error: {}", e),
        }
    }
}
