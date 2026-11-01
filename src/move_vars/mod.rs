//! src/move_vars/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Moving values between variables",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Move Vars");

        definition::index(true);
    }
}
