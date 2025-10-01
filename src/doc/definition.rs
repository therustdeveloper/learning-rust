//! src/doc/definition.rs
pub fn index(show: bool) {
    if show {
        println!("-- Documentation Definition");
        let mut x = 6;
        x = add_one(x);
        println!("x value: {x}");
    }
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = common::doc::definition::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// run cargo doc --open
// run cargo doc to generate the documentation
