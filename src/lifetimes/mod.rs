//! src/lifetimes/mod.rs
pub mod activity;
pub mod anonymous_lifetime;
pub mod compare_activity;
pub mod definition;
pub mod demo;
pub mod example;
pub mod multiple_lifetimes;
pub mod named_lifetime;
pub mod zero_copy;

crate::tui_module! {
    description: "Lifetimes and lifetime annotations",
    category: Intermediate,
    sections_only,
}
