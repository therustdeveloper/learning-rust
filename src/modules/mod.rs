//! src/modules/mod.rs
pub mod inline_modules_activity;
pub mod inline_modules_demo;
mod inventory;
pub mod inventory_program;
pub mod mod_basics;
mod orders;

pub fn index(show: bool) {
    if show {
        println!("\n-- Modules");

        inline_modules_demo::index(false);
        inline_modules_activity::index(false);
        mod_basics::index(false);
        inventory_program::index(false);
    }
}
