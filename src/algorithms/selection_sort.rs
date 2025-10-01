//! src/algorithms/selection_sort.rs
pub fn index(show: bool) {
    if show {
        println!("--- Selection Sort ---");
        println!("Sort numbers ascending");
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);
        selection_sort(&mut numbers);
        println!("After: {:?}", numbers);

        println!("Sort strings alphabetically");
        let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
        println!("Before: {:?}", strings);
        selection_sort(&mut strings);
        println!("After: {:?}", strings);
        println!(" ");
    }
}

fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}
