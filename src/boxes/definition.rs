//! src/boxes/definition.rs
use crate::boxes::dereference::box_dereference;

pub fn index(show: bool) {
    if show {
        println!("-- Using Box to Store Data on Heap");
        store_i32_on_heap();

        println!("-- Box Dereference");
        box_dereference();
    }
}

fn store_i32_on_heap() {
    let b = Box::new(5);
    println!("b = {b}");
}
