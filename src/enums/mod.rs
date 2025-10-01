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

pub fn index(show: bool) {
    if show {
        print_title("Enums");

        // Enum Demo
        demo::index(false);

        // Enum Definition
        definition::index(false);

        // Option Enums
        options::index(false);

        // Matches
        matches::index(false);

        // Enum Multi Array
        enum_multi_array::index(false);

        // Enum with Match
        enum_with_match::index(false);

        // Enum with Data
        enum_with_data::index(false);

        // Event enum
        event::index(false);

        // Enum holding data
        hold_data::index(false);

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
