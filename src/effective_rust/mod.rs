use crate::print_title;

pub mod types;

crate::tui_module! {
    description: "Effective Rust: types",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        print_title("Effective Rust");

        types::index(false);
    }
}
