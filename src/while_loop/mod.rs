//! src/while_loop/mod.rs
pub mod definition;
pub mod while_let;

pub fn master(show: bool) {
    if show {
        println!("\n-- While Loops");

        definition::master(false);

        while_let::index(true);
    }
}
