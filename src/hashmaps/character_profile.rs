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

        // we also can get a single value with get associated function
        match profile.get("name") {
            Some(value) => match value {
                CharacterValue::Name(name) => {
                    println!("the name is: {}", name);
                }
                _ => panic!("name should be a string"),
            },
            None => {
                println!("name is not present");
            }
        }

        // how to get all the values of a HashMap
        for (key, value) in &profile {
            match value {
                CharacterValue::Name(name) => println!("{}: {}", key, name),
                CharacterValue::Age(age) => println!("{}: {}", key, age),
                CharacterValue::Items(items) => println!("{}: {:?}", key, items),
            }
        }

        println!("{:?}", profile);
    }
}
