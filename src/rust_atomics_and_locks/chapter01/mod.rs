//! src/rust_atomics_and_locks/chapter01/mod.rs
mod thread_basics;
mod thread_closure;
mod thread_join;
mod thread_leaking;
mod thread_result;
mod thread_scoped;

pub fn master(show: bool) {
    if show {
        println!("---Chapter 01");

        thread_basics::master(false);
        thread_join::master(false);
        thread_closure::master(false);
        thread_result::master(false);
        thread_scoped::master(false);
        thread_leaking::master(true);
    }
}
