//! src/slices/mod.rs
use crate::print_title;
pub mod activity;
pub mod array_slices;
pub mod definition;
pub mod mutable_slice;
pub mod reference_array;

pub fn index(show: bool) {
    if show {
        print_title("Slices");

        activity::slices_activity(false);
        definition::index(false);
        array_slices::index(false);
        reference_array::index(false);
        mutable_slice::index(false);
    }
}
