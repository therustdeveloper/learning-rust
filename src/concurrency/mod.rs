//! src/concurrency/mod.rs
pub mod definition;
pub mod message_passing;
pub mod scoped_threads;
pub mod sharing_state_concurrently;

pub fn master(show: bool) {
    if show {
        println!("--- Concurrency");
        definition::master(false);
        message_passing::master(false);
        sharing_state_concurrently::master(false);
        scoped_threads::master(true);
    }
}
