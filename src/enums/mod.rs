//! src/enums/mod.rs
use crate::print_title;

pub mod cards;
pub mod catching_multiple_values;
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

pub fn master(show: bool) {
    if show {
        print_title("Enums");

        // Enum Demo
        demo::master(false);

        // Enum Definition
        definition::master(false);

        // Option Enums
        options::index(false);

        // Matches
        matches::master(false);

        // Enum Multi Array
        enum_multi_array::master(false);

        // Enum with Match
        enum_with_match::master(false);

        // Enum with Data
        enum_with_data::master(false);

        // Event enum
        event::master(false);

        // Enum holding data
        hold_data::master(false);

        // German Months
        german_months::index(false);

        // Cards Demo
        cards::index(false);

        // Payment Method Type
        payment_method_type::index(false);

        // Enums with Enums
        enums_with_enums::index(false);

        // Enums and Functions
        enums_and_functions::index(false);

        // Catching multiple values
        catching_multiple_values::index(false);

        // Match Exact Value
        match_exact_value::index(false);

        // Enum Emulator
        emulator::index(false);

        // Result Enum
        result_enum::index(false);
    }
}
