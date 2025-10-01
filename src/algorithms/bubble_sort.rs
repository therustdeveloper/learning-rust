//! src/algorithms/bubble_sort.rs
pub fn index(show: bool) {
    if show {
        println!("--- Bubble Sort ---");
        println!("Sort numbers ascending");
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);
        bubble_sort(&mut numbers);
        println!("After: {:?}", numbers);

        println!("Sort strings alphabetically");
        let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
        println!("Before: {:?}", strings);
        bubble_sort(&mut strings);
        println!("After: {:?}", strings);
        println!(" ");
    }
}

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
