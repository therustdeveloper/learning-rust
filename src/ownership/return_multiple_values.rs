//! src/ownership/return_multiple_values.rs
pub fn index(show: bool) {
    if show {
        let s1 = String::from("hallo");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
