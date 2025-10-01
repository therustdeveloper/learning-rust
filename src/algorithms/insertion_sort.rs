//! src/algorithms/insertion_sort.rs
pub fn index(show: bool) {
    if show {
        println!("--- Insertion Sort ---");
        println!("Sort numbers ascending");
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);
        insertion_sort(&mut numbers);
        println!("After: {:?}", numbers);

        println!("Sort strings alphabetically");
        let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
        println!("Before: {:?}", strings);
        insertion_sort(&mut strings);
        println!("After: {:?}", strings);
        println!(" ");
    }
}

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
