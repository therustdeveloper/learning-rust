//! src/algorithms/mod.rs
pub mod bubble_sort;
pub mod cocktail_sort;
pub mod comb_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

crate::tui_module! {
    description: "Classic sorting algorithms",
    category: DataStructures,
    sections_only,
}
