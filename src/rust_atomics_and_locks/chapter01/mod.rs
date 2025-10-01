//! src/rust_atomics_and_locks/chapter01/mod.rs
pub mod thread_basics;
pub mod thread_closure;
pub mod thread_join;
pub mod thread_leaking;
pub mod thread_result;
pub mod thread_scoped;

pub fn index(show: bool) {
    if show {
        println!("---Chapter 01");

        thread_basics::index(false);
        thread_join::index(false);
        thread_closure::index(false);
        thread_result::index(false);
        thread_scoped::index(false);
        thread_leaking::index(true);
    }
}
