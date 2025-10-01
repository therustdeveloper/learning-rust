//! src/results/mod.rs
pub mod activity;
pub mod definition;
pub mod question_mark_operator;

pub fn index(show: bool) {
    if show {
        println!("\n-- Results");

        definition::index(false);

        activity::index(false);

        question_mark_operator::index(true);
    }
}
