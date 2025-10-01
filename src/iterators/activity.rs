//! src/iterators/activity.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Activity Using Iterators");

        let data: Vec<_> = [1, 2, 3, 4, 5, 6, 7]
            .iter()
            .map(|num| num * 3)
            .filter(|num| num > &10)
            .collect();

        for num in data {
            println!("{:?}", num);
        }
    }
}
