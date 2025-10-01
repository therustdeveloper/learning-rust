//! src/iterators/find_position.rs
pub fn index(show: bool) {
    if show {
        println!("Iterators with find and position");

        let num_vec = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        println!("{:?}", num_vec.iter().find(|number| *number % 3 == 0));
        println!("{:?}", num_vec.iter().position(|number| *number % 3 == 0));
        println!("{:?}", num_vec.iter().find(|number| *number % 11 == 0));
        println!("{:?}", num_vec.iter().position(|number| *number % 11 == 0));
    }
}
