//! src/api_mutex/mod.rs
pub mod definition;
pub mod mutexes;
pub mod thread_parking;

crate::tui_module! {
    description: "The API of Mutex<T> and thread parking",
    category: Concurrency,
    sections_only,
}
