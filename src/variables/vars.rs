//! src/variables/variables.rs
pub fn index(show: bool) {
    if show {
        println!("-- Variables --");

        let apples = 50;
        println!("Number of apples {}", apples);

        let oranges = 77;
        println!("Number of oranges {}", oranges);

        println!("I have {} apples and {} oranges", apples, oranges);

        let fruits = apples + oranges;
        println!("Number of fruits {}", fruits);

        // mutable variables
        let mut gym_reps = 7;
        println!("Number of gym reps {}", gym_reps);

        gym_reps = 77;
        println!("Number of gym reps changed to {}", gym_reps);

        // variable shadowing
        let grams_of_protein = "100.345";
        println!(
            "Number of grams_of_protein as a string {}",
            grams_of_protein
        );

        let grams_of_protein = 100.345;
        println!("Number of grams_of_protein as a float {}", grams_of_protein);
    }
}
