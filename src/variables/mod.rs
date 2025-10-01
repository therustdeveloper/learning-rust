//! src/variables/mod.rs
pub mod constants;
pub mod vars;

use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("Variables and Mutability");

        vars::index(false);
        constants::index(false);
    }
}
