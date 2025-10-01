//! src/hashmaps/hashmap_get.rs
use std::collections::HashMap;

pub fn index(show: bool) {
    if show {
        println!("--- HashMap Get Method\n");

        let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
        let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielfeld"];
        let mut city_hashmap = HashMap::new();

        for city in canadian_cities {
            city_hashmap.insert(city, "Canada");
        }

        for city in german_cities {
            city_hashmap.insert(city, "Germany");
        }

        println!("{:?}", city_hashmap["Bielfeld"]);
        println!("{:?}", city_hashmap["Bielfeldd"]);
    }
}
