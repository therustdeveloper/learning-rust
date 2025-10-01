//! src/iterators/rev.rs
pub fn index(show: bool) {
    if show {
        println!("Iterators with rev method");

        let mut big_vec = vec![6; 100];
        big_vec.push(5);

        println!("{:?}", big_vec.contains(&5));
    }
}
