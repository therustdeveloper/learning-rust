//! src/enums/enum_multi_array.rs
enum SomeValue {
    StringValue(String),
    IntValue(i32),
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Enums Multi Arrays");
        let multi_array: [SomeValue; 4] = [
            SomeValue::StringValue(String::from("one")),
            SomeValue::IntValue(3),
            SomeValue::StringValue(String::from("three")),
            SomeValue::IntValue(7),
        ];

        for i in multi_array {
            match i {
                SomeValue::StringValue(data) => {
                    println!("The string is {}", data);
                }
                SomeValue::IntValue(data) => {
                    println!("The int is {}", data);
                }
            }
        }
    }
}
