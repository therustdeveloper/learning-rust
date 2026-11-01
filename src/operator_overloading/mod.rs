//! src/operator_overloading/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Operator overloading with std::ops traits",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Operator Overloading");

        definition::index(true);
    }
}
