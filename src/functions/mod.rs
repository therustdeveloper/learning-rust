//! src/functions/mod.rs
pub mod blocks_in_functions;
pub mod definition;
pub mod func_return_option;
pub mod nested_functions;

pub fn index(show: bool) {
    if show {
        println!("-- Functions --");

        definition::index(false);
        blocks_in_functions::index(false);
        func_return_option::index(false);
        nested_functions::index(false);
    }
}
