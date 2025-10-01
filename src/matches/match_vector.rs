//! src/matches/match_vector.rs
pub fn index(show: bool) {
    if show {
        println!("--- Match in vector\n");

        let scores = [3, 4, 5, 6, 7];

        for index in 0..10 {
            if let Some(number) = scores.get(index) {
                println!("The number is: {number}");
            }
        }
    }
}
