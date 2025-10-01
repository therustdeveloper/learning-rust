//! src/hashmaps/hashmap_loop.rs
use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

pub fn index(show: bool) {
    if show {
        println!("--- Hashmap in a for loop\n");

        let mut tallinn = City {
            name: "Tallinn".to_string(),
            population: HashMap::new(),
        };

        tallinn.population.insert(2020, 437_619);
        tallinn.population.insert(1372, 3_250);
        tallinn.population.insert(1851, 24_000);

        for (year, population) in tallinn.population {
            println!(
                "In {year}, {:#} had a population of {population}.",
                tallinn.name
            );
        }
    }
}
