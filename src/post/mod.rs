//! src/post/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Post");

        definition::index(true);
    }
}
