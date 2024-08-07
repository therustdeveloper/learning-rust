mod definition;
mod message_passing_between_threads;
mod mutexes;
mod preventing_race_condition;
mod sharing_data_between_threads;

pub fn master(show: bool) {
    if show {
        println!("\n-- Threads");

        definition::master(false);

        preventing_race_condition::master(false);

        mutexes::master(false);

        sharing_data_between_threads::master(false);

        message_passing_between_threads::master(false);
    }
}
