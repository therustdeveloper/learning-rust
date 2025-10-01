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

pub fn index(show: bool) {
    if show {
        println!("\n-- Threads");

        definition::index(false);

        preventing_race_condition::index(false);

        mutexes::index(false);

        sharing_data_between_threads::index(false);

        message_passing_between_threads::index(false);

        move_data_between_threads::index(false);

        threads_in_array::index(false);

        scoped_threads::index(false);

        sending_multiple_values::index(false);

        multithreading::index(false);
    }
}
