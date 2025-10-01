//! src/iterators/iter_in_collections.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        // Most collection types provide iter and iter_mut methods that return the natural
        // iterators over the type, producing a shared or mutable reference to each item.
        print_title("Iter in Collections");

        let v = [1, 7, 14, 21, 28, 35, 42];
        let mut iterator = v.iter();

        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&7));
        assert_eq!(iterator.next(), Some(&14));
        assert_eq!(iterator.next(), Some(&21));
        assert_eq!(iterator.next(), Some(&28));
        assert_eq!(iterator.next(), Some(&35));
        assert_eq!(iterator.next(), Some(&42));
        assert_eq!(iterator.next(), None);

        for_each_iter(false);
    }
}

pub fn for_each_iter(show: bool) {
    if show {
        ["doves", "hens", "birds"]
            .iter()
            .zip(["turtle", "french", "calling"])
            .zip(2..5)
            .rev()
            .map(|((item, kind), quantity)| format!("{}{}{}", quantity, kind, item))
            .for_each(|gift| {
                println!("You have received: {}", gift);
            });
    }
}
