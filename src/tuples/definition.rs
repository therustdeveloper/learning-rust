//! src/tuples/definition.rs
enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

pub fn index(show: bool) {
    if show {
        demo(show);
        person_info(show);
        receive_tuple(show);
    }
}

fn demo(show: bool) {
    if show {
        println!("\n--- Definition");

        // Option 1
        let numbers = one_two_three();
        // Option 2
        let (x, y, z) = one_two_three();

        println!("{}, {}", x, numbers.0);
        println!("{}, {}", y, numbers.1);
        println!("{}, {}", z, numbers.2);

        // Combined with Enums
        let (employee, access) = ("Jake", Access::Full);
        println!("employee name: {}", employee);

        match access {
            Access::Full => println!("employee has full access"),
        };

        let tup = (700, 7.0, 7);
        println!("{:?}", tup);

        let (x, y, z) = tup;
        println!("x: {x}, y: {y}, z: {z}");
    }
}

fn person_info(show: bool) {
    if show {
        let person: (String, u32, bool) = ("Alice".to_string(), 30, true);
        println!(
            "Name: {}, Age: {}, Active: {}",
            person.0, person.1, person.2
        );

        // pattern matching
        let (name, age, is_registered) = person;
        println!("Name: {}, Age: {}, Active: {}", name, age, is_registered);
    }
}

fn receive_tuple(show: bool) {
    if show {
        let (area, perimiter) = calculate_area_and_perimeter(7.7, 77.7);
        println!("Area #{}, Perimeter: {}", area, perimiter);
    }
}

fn calculate_area_and_perimeter(length: f64, width: f64) -> (f64, f64) {
    let area = length * width;
    let perimeter = 2.0 * (length * width);
    (area, perimeter)
}
