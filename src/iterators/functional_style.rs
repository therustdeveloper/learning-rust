//! src/iterators/functional_style.rs
use std::vec;

pub fn index(show: bool) {
    if show {
        println!("Functional style");

        simple_example(false);

        chain_many_methods(false);
    }
}

fn simple_example(show: bool) {
    if show {
        let new_vec = (1..).take(10).collect::<Vec<i32>>();
        println!("{new_vec:?}");
    }
}

fn chain_many_methods(show: bool) {
    if show {
        let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
        println!("{new_vec:?}");
    }
}
