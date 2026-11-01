//! src/interior_mutability/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Interior mutability with RefCell<T>",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        println!("Interior Mutability");

        definition::index(false);
    }
}
