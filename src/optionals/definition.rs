//! src/optionals/definition.rs
struct Student {
    name: String,
    locker: Option<i32>,
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Optional Definition");

        let mary = Student {
            name: "Mary".to_string(),
            locker: Some(3),
        };

        println!("student: {:?}", mary.name);
        match mary.locker {
            Some(num) => println!("locker number: {:?}", num),
            None => println!("no locker assigned"),
        }
    }
}
