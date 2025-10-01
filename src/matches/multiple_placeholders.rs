//! src/matches/multiple_placeholders.rs
pub fn index(show: bool) {
    if show {
        println!("--- Matches with several placeholders\n");

        let first = (200, 0, 0);
        let second = (50, 50, 50);
        let third = (200, 50, 0);

        match_colors(first);
        match_colors(second);
        match_colors(third);
    }
}

fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has at least 10"),
    }
}
