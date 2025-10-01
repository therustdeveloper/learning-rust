//! src/iterators/demo.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Operators");

        //let numbers = vec![0, 1, 2, 3, 4, 5, 6];

        /*let mut plus_one = vec![];
        for num in numbers {
            plus_one.push(num + 1);
        }
        */

        let plus_one: Vec<_> = [0, 1, 2, 3, 4, 5, 6].iter().map(|num| num + 1).collect();

        println!("New vector: {:?}", plus_one);
    }
}
