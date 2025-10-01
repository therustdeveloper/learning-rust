//! src/algorithms/mod.rs
pub mod bubble_sort;
pub mod cocktail_sort;
pub mod comb_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

pub fn index(show: bool) {
    if show {
        // Quick Sort
        quick_sort::index(false);

        // Bubble Sort
        bubble_sort::index(false);

        // Insertion Sort
        insertion_sort::index(false);

        // Selection Sort
        selection_sort::index(false);

        // Merge Sort
        merge_sort::index(false);

        // Cocktail Sort
        cocktail_sort::index(false);

        // Comb Sort
        comb_sort::index(false);
    }
}
