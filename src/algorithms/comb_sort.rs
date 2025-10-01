//! src/algorithms/comb_sort.rs
use crate::print_title;

fn comb_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    let mut gap = arr.len();
    let shrink_factor = 1.3;
    let mut sorted = false;

    while !sorted {
        // Update the gap value for the next iterator
        gap = (gap as f64 / shrink_factor).floor() as usize;

        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        let mut i = 0;
        while i + gap < arr.len() {
            if arr[i] > arr[i + gap] {
                // swap elements
                arr.swap(i, i + gap);
                sorted = false;
            }
            i += 1;
        }
    }
}

pub fn index(show: bool) {
    if show {
        print_title("Comb Sort");

        let mut arr = vec![5, 3, 7, 4, 6, 1, 2];
        comb_sort(&mut arr);
        println!("Sorted array: {:?}", arr);
    }
}
