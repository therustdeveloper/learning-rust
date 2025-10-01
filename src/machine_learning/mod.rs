//! src/machine_learning/mod.rs
use crate::print_title;
pub mod reading_csv_file;

pub fn master(show: bool) {
    if show {
        print_title("Machine Learning");

        let _ = reading_csv_file::index(false);
    }
}
