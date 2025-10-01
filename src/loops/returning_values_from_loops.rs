//! src/loops/returning_values_from_loops.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Returning Values From Loops ---");

        let mut counter = 0;

        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 7;
            }
        };

        println!("The result is {result}");
    }
}
