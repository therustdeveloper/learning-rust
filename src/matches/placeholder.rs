//! src/matches/placeholder.rs
pub fn index(show: bool) {
    if show {
        println!("--- Match with Placeholder");

        let dice_roll = 9;

        match dice_roll {
            3 => add_fancy_shirt(),
            7 => remove_fancy_shirt(),
            _ => reroll(),
        }
    }
}

fn add_fancy_shirt() {
    println!("Add a fancy t-shirt");
}

fn remove_fancy_shirt() {
    println!("Remove a fancy t-shirt");
}

fn reroll() {
    println!("Re-roll");
}
