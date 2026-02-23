//! src/variables/mod.rs
pub mod constants;
pub mod copying_vars;
pub mod moving_vars;
pub mod vars;

use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("Variables and Mutability");

        vars::index(false);
        constants::index(false);
        copying_vars::index(false);
        moving_vars::index(false);
    }
}
