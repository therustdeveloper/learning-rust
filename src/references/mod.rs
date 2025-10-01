//! src/references/mod.rs
pub mod basic_reference;
pub mod definition;
pub mod mut_reference;
pub mod ref_of_references;

pub fn index(show: bool) {
    if show {
        println!("\n-- References");

        definition::index(false);

        ref_of_references::index(false);

        basic_reference::index(false);

        mut_reference::index(false);
    }
}
