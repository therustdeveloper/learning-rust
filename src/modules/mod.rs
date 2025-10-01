//! src/modules/mod.rs
pub mod inline_modules_activity;
pub mod inline_modules_demo;
pub mod mod_basics;

pub fn index(show: bool) {
    if show {
        println!("\n-- Modules");

        inline_modules_demo::index(false);
        inline_modules_activity::index(false);
        mod_basics::index(false);
    }
}
