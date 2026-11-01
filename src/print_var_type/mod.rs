//! src/print_var_type/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Printing the type of a variable",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- Print Var Type");

        definition::index(true);
    }
}
