//! src/functions/blocks_in_functions.rs

pub fn index(show: bool) {
    if show {
        let multiplier = 7;

        let calculation = {
            let value = 4 + 3;
            value * multiplier
        };

        println!("{calculation}");
    }
}
