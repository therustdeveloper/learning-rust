//! src/generics/mod.rs
pub mod activity;
pub mod conveyor_belt;
pub mod definition;
pub mod demo;
pub mod generic_add;
pub mod generic_enums;
pub mod generic_functions;
pub mod generic_implementation;
pub mod generic_structs;
pub mod generic_structures;
pub mod generic_with_where;
pub mod introduction;
pub mod multiple_generics;
pub mod question_operator;
pub mod vehicle;

crate::tui_module! {
    description: "Generic functions, structs, enums and implementations",
    category: Intermediate,
    sections_only,
}
