//! src/iterators/iterator_next.rs
pub fn index(show: bool) {
    if show {
        println!("Iterators with next");

        let my_vec = ['a', 'b', 'c', 'd'];

        let mut my_vec_iter = my_vec.iter();

        assert_eq!(my_vec_iter.next(), Some(&'a'));
        assert_eq!(my_vec_iter.next(), Some(&'b'));
        assert_eq!(my_vec_iter.next(), Some(&'c'));
        assert_eq!(my_vec_iter.next(), Some(&'d'));
    }
}
