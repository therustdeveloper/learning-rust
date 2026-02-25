use crate::print_title;

pub mod types;

pub fn index(show: bool) {
    if show {
        print_title("Effective Rust");

        types::index(false);
    }
}
