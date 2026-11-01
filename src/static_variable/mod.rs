//! src/static_variable/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Accessing and modifying static variables",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Static Variable");

        definition::index(true);
    }
}
