pub mod definition;

crate::tui_module! {
    description: "Borrowing values with references",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        println!("-- Borrowing --");
    }
}
