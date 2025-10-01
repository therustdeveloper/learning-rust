//! src/rc_t/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- RC T");

        definition::index(true);
    }
}
