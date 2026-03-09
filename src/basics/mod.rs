//! src/basics/mod.rs
use crate::print_title;

pub mod arrays;
pub mod booleans;
pub mod borrowing;
pub mod branches;
pub mod characters;
pub mod clone;
pub mod compounds;
pub mod constants;
pub mod control_flow;
pub mod convert_values;
pub mod floats;
pub mod functions;
pub mod if_let;
pub mod integers;
pub mod loops;
pub mod match_statements;
pub mod num_operations;
pub mod recursion;
pub mod scalars;
pub mod scopes;
pub mod shadowing;
pub mod slices;
pub mod type_casting;
pub mod underscores;
pub mod unicode;
pub mod variables;

pub fn index(show: bool) {
    if show {
        print_title("BASICS");

        // Variables
        variables::index(true);

        // Constants
        constants::index(false);

        // Shadowing
        shadowing::index(false);

        // Scalar Types
        scalars::index(false);

        // Floating Points
        floats::index(false);

        // Numeric Operations
        num_operations::index(false);

        // Boolean Types
        booleans::index(false);

        // Character Types
        characters::index(false);

        // Tuple Types
        compounds::index(false);

        // Functions
        functions::index(false);

        // Control Flow
        control_flow::index(false);

        // Loops
        loops::index(false);

        // Slices
        slices::index(false);

        // Integers
        integers::index(false);

        // Arrays
        arrays::index(false);

        // Clone
        clone::index(false);

        // Borrowing
        borrowing::index(false);

        // Branches
        branches::index(false);

        // Using if in a let statement
        if_let::index(false);

        // Using underscores in numbers
        underscores::index(false);

        // Convert values from one to another
        convert_values::index(false);

        // Match Statement
        match_statements::index(false);

        // Recursion
        recursion::index(false);

        // Scopes
        scopes::index(false);

        // Type Casting
        type_casting::index(false);

        // unicode
        unicode::index(false);
    }
}
