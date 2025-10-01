//! src/tree_data_structure/definition.rs
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn index(show: bool) {
    if show {
        println!("-- Tree Data Structure: A Node with Child Nodes");
        definition();
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn definition() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf value = {:?}, leaf parent = {:?}, leaf children = {:?}",
        leaf.value,
        leaf.parent.borrow().upgrade(),
        leaf.children,
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
