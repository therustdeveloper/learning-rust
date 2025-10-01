//! src/rust_atomics_and_locks/mod.rs
pub mod chapter01;

pub fn index(show: bool) {
    if show {
        println!("--- Rust Atomics and Locks Book");

        chapter01::index(false);
    }
}
