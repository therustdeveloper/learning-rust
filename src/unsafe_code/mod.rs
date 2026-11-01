//! src/unsafe_code/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Unsafe Rust",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Unsafe Code");

        definition::index(true);
    }
}
