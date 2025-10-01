//! src/hashmaps/activity.rs
use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

pub fn index(show: bool) {
    if show {
        println!("\n--- HashMap Activity");

        let mut lockers = HashMap::new();

        lockers.insert(
            1,
            Contents {
                content: "keys".to_owned(),
            },
        );
        lockers.insert(
            2,
            Contents {
                content: "bag".to_owned(),
            },
        );
        lockers.insert(
            3,
            Contents {
                content: "glasses".to_owned(),
            },
        );
        lockers.insert(
            4,
            Contents {
                content: "short".to_owned(),
            },
        );

        for (locker_number, content_item) in lockers.iter() {
            println!(
                "locker number: {}, content: {:?}",
                locker_number, content_item.content
            );
        }
    }
}
