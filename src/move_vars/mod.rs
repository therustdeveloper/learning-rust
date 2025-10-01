//! src/move_vars/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Move Vars");

        definition::index(true);
    }
}
