//! src/methods/mod.rs
pub mod associated_functions;
pub mod definition;
pub mod multiple_impl_blocks;
pub mod parameters;

crate::tui_module! {
    description: "Methods, associated functions and impl blocks",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        // Methods Definition
        definition::index(false);

        // Methods with Parameters
        parameters::index(false);

        // Associated Functions
        associated_functions::index(false);

        // Multiple impl Blocks
        multiple_impl_blocks::index(false);
    }
}
