//! src/rc_t/mod.rs
pub mod definition;

crate::tui_module! {
    description: "Reference counting with Rc<T>",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        println!("\n-- RC T");

        definition::index(true);
    }
}
