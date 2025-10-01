//! src/while_loop/mod.rs
pub mod definition;
pub mod while_let;

pub fn index(show: bool) {
    if show {
        println!("\n-- While Loops");

        definition::index(false);

        while_let::index(true);
    }
}
