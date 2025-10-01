//! src/decisions/definition.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Making Decisions with Rust");

        // show if statement
        if_statement();

        // control the flow with variables
        if_with_vars();
    }
}

fn if_statement() {
    println!("\n--- If Statement");

    let age = 15;
    if age >= 21 {
        println!("ok to purchase");
    } else {
        println!("cannot purchase");
    }
}

fn if_with_vars() {
    println!("\n--- Checking a variable value");

    let allowed = false;

    if allowed {
        println!("It's allowed");
    } else {
        println!("Not allowed");
    }
}
