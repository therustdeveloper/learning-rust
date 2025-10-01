pub fn index(show: bool) {
    if show {
        let action_hero = String::from("Arnold Schwarzenegger");
        let first_name = &action_hero[0..6];
        println!("First Name: {first_name}");

        let last_name = &action_hero[7..21];
        println!("Last Name: {last_name}");

        slice_lenght(false);

        do_hero_stuff(&action_hero);
        let another_action_hero = "Sylvester Stallone";
        do_hero_stuff(&another_action_hero.to_string());

        // the cool stuff with slices
        do_hero_stuff_slice(&action_hero);
        do_hero_stuff_slice(another_action_hero);
    }
}

fn slice_lenght(show: bool) {
    if show {
        let food = "🍕";
        println!("Lenght food: {}", food.len());
    }
}

fn do_hero_stuff(hero_name: &String) {
    println!("{hero_name} saves the day");
}

fn do_hero_stuff_slice(hero_name: &str) {
    println!("{hero_name} saves the day");
}
