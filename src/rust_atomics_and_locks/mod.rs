//! src/rust_atomics_and_locks/mod.rs
pub mod chapter01;

crate::tui_module! {
    description: "Rust Atomics and Locks",
    category: Concurrency,
}

pub fn index(show: bool) {
    if show {
        println!("--- Rust Atomics and Locks Book");

        chapter01::index(false);
    }
}
