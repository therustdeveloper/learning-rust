//! src/machine_learning/mod.rs
use crate::print_title;
pub mod reading_csv_file;

crate::tui_module! {
    description: "Machine learning: reading CSV files with Polars",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        print_title("Machine Learning");

        let _ = reading_csv_file::index(false);
    }
}
