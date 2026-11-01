//! src/modules/mod.rs
pub mod inline_modules_activity;
pub mod inline_modules_demo;
mod inventory;
pub mod inventory_program;
pub mod mod_basics;
mod orders;

crate::tui_module! {
    description: "Organizing code with modules",
    category: Intermediate,
    sections_only,
}
