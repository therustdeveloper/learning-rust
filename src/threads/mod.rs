//! src/threads/mod.rs
pub mod definition;
pub mod message_passing_between_threads;
pub mod move_data_between_threads;
pub mod multithreading;
pub mod mutexes;
pub mod preventing_race_condition;
pub mod scoped_threads;
pub mod sending_multiple_values;
pub mod sharing_data_between_threads;
pub mod threads_in_array;

crate::tui_module! {
    description: "Threads, shared data and race conditions",
    category: Concurrency,
    sections_only,
}
