//! src/mutable_reference/definition.rs
pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

// move the value behind a mutable reference
pub fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty
    // let was = *s;
    // but this is:
    let was = std::mem::take(s);
    // so this is:
    *s = was;
    // we can exchange values behind &mut:
    let mut r = Box::new(84); // is r put in the heap?
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}

pub fn index(show: bool) {
    if show {
        mut_definition(false);

        mut_reference(false);
    }
}

fn mut_definition(show: bool) {
    if show {
        println!("Mutable Reference");
        let mut s = Box::new(42);
        println!("value of s: {}", s);
        replace_with_84(&mut s);
        println!("value of s: {}", s);
    }
}

fn mut_reference(show: bool) {
    if show {
        let x = 77;
        println!("x is of type: {:?}", x.type_name());

        let mut y = &x;
        println!("y is of type: {:?}", y.type_name());

        let z = &mut y;
        println!("z is of type: {:?}", z.type_name());
    }
}
