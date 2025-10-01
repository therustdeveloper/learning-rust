//! src/rc_t/definition.rs
use crate::rc_t::definition::List::Cons;
use crate::rc_t::definition::List::Nil;
use std::rc::Rc;

pub fn index(show: bool) {
    if show {
        println!("-- Working with Rc<T>");
        definition();
    }
}

#[allow(unused)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn definition() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after createing a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    } // here c goes out of scope
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
