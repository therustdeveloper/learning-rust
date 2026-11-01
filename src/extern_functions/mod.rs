//! src/extern_functions/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Calling external code with extern functions",
    category: System,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Extern Functions");

        definition::index(true);
    }
}
