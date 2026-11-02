//! src/enums/mod.rs
pub mod cards;
pub mod catching_multiple_values;
pub mod color;
pub mod definition;
pub mod demo;
pub mod emulator;
pub mod enum_multi_array;
pub mod enum_with_data;
pub mod enum_with_match;
pub mod enums_and_functions;
pub mod enums_with_enums;
pub mod event;
pub mod german_months;
pub mod hold_data;
pub mod match_exact_value;
pub mod matches;
pub mod options;
pub mod payment_method_type;
pub mod result_enum;

crate::tui_module! {
    description: "Enums, data-carrying variants and matching",
    category: Basics,
    sections_only,
}
