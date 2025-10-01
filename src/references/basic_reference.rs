//! src/references/basic_reference.rs
pub fn index(show: bool) {
    if show {
        println!("--- Basic Reference");

        let country = String::from("Austria");

        let ref_one = &country;
        let ref_two = &country;

        println!("ref_one: {}", ref_one);
        println!("ref_two: {}", ref_two);
    }
}
