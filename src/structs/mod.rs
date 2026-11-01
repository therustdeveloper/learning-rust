//! src/structs/mod.rs
pub mod builder_pattern;
pub mod coffee;
pub mod definition;
pub mod file;
pub mod human;
pub mod integrate_struct_enum_match;
pub mod methods;
pub mod person;
pub mod queue;
pub mod struct_update_syntax;
pub mod structs_owned;
pub mod tuple_structs;
pub mod unit_struct;

crate::tui_module! {
    description: "Structs, tuple structs and the builder pattern",
    category: Basics,
    sections_only,
}
