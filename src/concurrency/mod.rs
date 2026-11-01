//! src/concurrency/mod.rs
pub mod definition;
pub mod message_passing;
pub mod scoped_threads;
pub mod sharing_state_concurrently;

crate::tui_module! {
    description: "Fearless concurrency: threads, message passing and shared state",
    category: Concurrency,
    sections_only,
}
