//! src/extern_functions/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Extern Functions");

        definition::index(true);
    }
}
