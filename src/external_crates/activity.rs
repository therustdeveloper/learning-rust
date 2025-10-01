//! src/external_crates/activity.rs
use chrono::prelude::*;

pub fn index(show: bool) {
    if show {
        println!("\n--- Activity External Crates");

        let utc: DateTime<Utc> = Utc::now();
        println!("{}", utc);

        let local: DateTime<Local> = Local::now();
        println!("{}", local);

        // print a formatted version
        println!("{}", local.format("%Y-%m-%d %H:%M:%S"));
    }
}
