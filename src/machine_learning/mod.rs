//! src/machine_learning/mod.rs
pub mod normalization;
pub mod reading_csv_file;

crate::tui_module! {
    description: "Machine learning: reading CSV files with Polars",
    category: Advanced,
    sections_only,
}
