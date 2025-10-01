//! src/references/definition.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- References Definition");

        basics();

        basics_mutable();
    }
}

fn basics() {
    println!("\n--- Immutable References");

    let x: i32 = 42; // x owns the value
    let r = &x; // r borrow the value
    let v: i32 = *r;

    println!("x: {}", x);
    println!("r: {}", r);
    println!("v: {}", v);

    println!("x: {}", x); // this confirms that x is still owning the value
}

fn basics_mutable() {
    println!("\n--- Mutable References");

    let mut x: i32 = 77;
    let r = &x;
    let v: i32 = *r;

    println!("x: {}", x);
    println!("r: {}", r);
    println!("v: {}", v);

    println!("x: {}", x); // this confirms that x is still owning the value

    let m = &mut x;
    println!("m: {}", m);

    *m = 777;
    println!("m: {}", m); // this borrows the value

    println!("x: {}", x); // the ownership is returned to x
}
