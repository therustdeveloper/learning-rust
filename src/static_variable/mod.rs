//! src/static_variable/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Static Variable");

        definition::index(true);
    }
}
