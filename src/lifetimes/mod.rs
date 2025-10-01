//! src/lifetimes/mod.rs
use crate::print_title;

pub mod activity;
pub mod anonymous_lifetime;
pub mod compare_activity;
pub mod definition;
pub mod demo;
pub mod example;
pub mod multiple_lifetimes;
pub mod named_lifetime;

pub fn index(show: bool) {
    if show {
        print_title("Lifetimes");

        definition::index(false);

        example::index(false);

        demo::index(false);

        activity::index(false);

        compare_activity::index(false);

        named_lifetime::master(false);

        anonymous_lifetime::master(false);

        multiple_lifetimes::index(false);
    }
}
