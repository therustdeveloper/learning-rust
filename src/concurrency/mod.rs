//! src/concurrency/mod.rs
pub mod definition;
pub mod message_passing;
pub mod scoped_threads;
pub mod sharing_state_concurrently;

pub fn index(show: bool) {
    if show {
        println!("--- Concurrency");

        definition::index(false);
        message_passing::index(false);
        sharing_state_concurrently::index(false);
        scoped_threads::index(true);
    }
}
