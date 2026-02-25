use crate::print_title;
pub mod item1;

pub fn index(show: bool) {
    if show {
        print_title("Effective Rust - Types");

        item1::index(false);
    }
}
