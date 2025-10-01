//! src/api_mutex/mod.rs
pub mod definition;
pub mod mutexes;
pub mod thread_parking;

pub fn index(show: bool) {
    if show {
        // Array Definition
        definition::index(false);

        // Mutex in Practice
        mutexes::index(false);

        // Thread Parking
        thread_parking::index(false);
    }
}
