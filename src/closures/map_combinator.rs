//! src/closures/map_combinator.rs
#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Map Combinator");

        let user_name = "sam";
        // because find_user returns an Option we can use map because Option implements it
        let user = find_user(user_name).map(|user_id| User {
            user_id,
            name: user_name.to_owned(),
        });

        match user {
            Some(user) => {
                println!("user id: {:?}", user.user_id);
                println!("user name: {:?}", user.name);
            }
            None => println!("user not found"),
        }
    }
}
