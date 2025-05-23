//! src/structs/definition.rs
#[derive(Debug)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

pub fn index(show: bool) {
    if show {
        let mut user1 = User {
            active: true,
            username: String::from("bellerophon"),
            email: String::from("bellerophon@example.com"),
            sign_in_count: 1,
        };

        user1.email = String::from("bellerophon@rustdeveloper.io");
        println!("{}", user1.username);
        println!("{} {}", user1.active, user1.sign_in_count);

        let mut user2 = build_user(String::from("chimera@example.com"), String::from("chimera"));

        user2.email = String::from("chimera@rustdeveloer.io");
        println!("{}", user2.username);

        // Working with Structs
        send_box();

        // Create Country Struct
        create_country();
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn send_box() {
    println!("\n-- Working with Structs");

    let container_box = ShippingBox {
        depth: 7,
        width: 7,
        height: 7,
    };

    let height = container_box.height;
    let depth = container_box.depth;
    let width = container_box.width;

    println!("container_box is {} height", height);
    println!("container_box is {} depth", depth);
    println!("container box is {} width", width);
}

fn create_country() {
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population,
        capital,
        leader_name,
    };

    println!("Country: {:?}", kalmykia);
    println!("Country Population: {:?}", kalmykia.population);
    println!("Country Capita: {:?}", kalmykia.capital);
    println!("Country Leader Name: {:?}", kalmykia.leader_name);
}
