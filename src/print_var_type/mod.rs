//! src/print_var_type/mod.rs
pub mod definition;

pub fn index(show: bool) {
    if show {
        println!("\n-- Print Var Type");

        definition::index(true);
    }
}
