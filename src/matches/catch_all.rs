//! src/matches/catch_all.rs
pub fn index(show: bool) {
    if show {
        println!("--- Catch-All Patterns");

        let dice_roll = 9;

        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
    }
}

fn add_fancy_hat() {
    println!("Add a fancy hat");
}

fn remove_fancy_hat() {
    println!("Remove a fancy hat");
}

fn move_player(num_spaces: u8) {
    println!("Move player {} spaces", num_spaces);
}
