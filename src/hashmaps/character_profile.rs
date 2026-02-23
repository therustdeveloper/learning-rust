use std::collections::HashMap;

#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>),
}

pub fn index(show: bool) {
    if show {
        let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

        profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
        profile.insert("age", CharacterValue::Age(27));
        profile.insert(
            "items",
            CharacterValue::Items(vec![
                "laptop".to_string(),
                "book".to_string(),
                "coat".to_string(),
            ]),
        );

        for (key, value) in &profile {
            match value {
                CharacterValue::Name(name) => println!("{}: {}", key, name),
                CharacterValue::Age(age) => println!("{}: {}", key, age),
                CharacterValue::Items(items) => println!("{}: {:?}", key, items),
            }
        }
    }
}
