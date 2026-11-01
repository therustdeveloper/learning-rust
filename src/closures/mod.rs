//! src/closures/mod.rs
pub mod closures_inside_methods;
pub mod definition;
pub mod demo;
pub mod map_collect;
pub mod map_combinator;
pub mod process_closure_thread;
pub mod simple_closure;
pub mod vars_outside_closure;

crate::tui_module! {
    description: "Closures, captured variables and combinators",
    category: Intermediate,
    sections_only,
}
