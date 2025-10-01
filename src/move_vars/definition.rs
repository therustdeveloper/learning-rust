//! src/move_vars/definition.rs
enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

// implementing borrow
// light cannot deleted here because it is just borrow the memory
fn display_light_with_borrow(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Moving Variables");

        let dull = Light::Dull;
        display_light(dull);
        //display_light(dull);
        // this doesn't compile because the owner of the variable dull is main
        // then is move into display_light, that becomes the owner of dull
        // once display_light finish its work, dull is destroyed.

        let bright = Light::Bright;
        display_light(bright);
        // exactly the same happens with bright variable, main is the owner
        // once the display_light function is call, the var bright is move to display_light

        // using borrow
        let dull2 = Light::Dull;
        display_light_with_borrow(&dull2);
        display_light_with_borrow(&dull2);
    }
}
