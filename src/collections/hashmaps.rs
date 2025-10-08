use std::collections::HashMap;

#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>),
}

pub fn index(show: bool) {
    if show {
        println!("--- HashMaps ---");

        let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

        profile.insert("name", CharacterValue::Name(String::from("Maxwell")));
        profile.insert("age", CharacterValue::Age(37));
        profile.insert("items", CharacterValue::Items(vec![
            String::from("laptop"),
            String::from("book"),
            String::from("coat"),
        ]));

        if let Some(CharacterValue::Name(name)) = profile.get("name") {
            println!("Name: {}", name);
        }
        if let Some(CharacterValue::Age(age)) = profile.get("age") {
            println!("Age: {}", age);
        }
        if let Some(CharacterValue::Items(items)) = profile.get("items") {
            for item in items {
                println!("- {}", item);
            }
        }
    }
}