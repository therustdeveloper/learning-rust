//! src/shadowing/definition.rs
pub fn index(show: bool) {
    if show {
        println!("--- Shadowing Definition");

        let country = String::from("Austria");
        let country_ref = &country;

        let country = 8; // shadowing country variable
        println!("country_ref: {} and new country: {}", country_ref, country);
    }
}
