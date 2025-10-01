//! src/hashmaps/definition.rs
use std::collections::HashMap;

#[allow(unused)]
#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>),
}

pub fn index(show: bool) {
    if show {
        println!("\n--- HashMaps Definition");

        let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

        profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
        profile.insert("age", CharacterValue::Age(32));
        profile.insert(
            "items",
            CharacterValue::Items(vec![
                "laptop".to_string(),
                "book".to_string(),
                "coat".to_string(),
            ]),
        );

        println!("{:?}", profile);

        match profile.get("name") {
            Some(value_data) => match value_data {
                CharacterValue::Name(name) => {
                    println!("the name is: {}", name);
                }
                _ => panic!("name should be a string"),
            },
            None => {
                println!("name is not present");
            }
        }

        // Working with HashMaps
        scores();

        // Updating a value based on the old value
        updating_value_based_on_old_value();
    }
}

fn scores() {
    println!("--- HashMap Scores ---");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 77);

    println!("{:?}", scores);

    println!("--- Accessing Values ---");

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The value of score is: {}", score);

    println!("--- Loop in the HashMap ---");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("--- Only insert if the key does not already have a value ---");
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

fn updating_value_based_on_old_value() {
    println!("--- Updating value based on the old value ---");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Value of map: {:?}", map);
}
