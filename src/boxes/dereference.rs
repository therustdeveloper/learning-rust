//! src/boxes/dereference.rs
pub fn box_dereference() {
    let x = 7;
    let y = Box::new(x);

    println!("Value of x: {x}");
    println!("Value that y is pointing to: {:?}", *y);

    assert_eq!(7, x);
    assert_eq!(7, *y);
}

pub fn index(show: bool) {
    if show {
        box_dereference();
    }
}
