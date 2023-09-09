pub mod constants;
pub mod scalar_types;
pub mod shadowing;
pub mod variables;
pub mod floating_point;
pub mod numeric_operations;
pub mod boolean_type;
pub mod character_type;
pub mod compound_types;
pub mod functions;
pub mod control_flow;
pub mod loops;
pub mod slices;
pub mod integers;

pub fn master(show: bool) {
    if show {
        common::print_title("BASICS");

        // Variables
        variables::master();

        // Constants
        constants::master();

        // Shadowing
        shadowing::master();

        // Scalar Types
        scalar_types::master();

        // Floating Points
        floating_point::master();

        // Numeric Operations
        numeric_operations::master();

        // Boolean Types
        boolean_type::master();

        // Character Types
        character_type::master();

        // Tuple Types
        compound_types::tuple_type();

        // Array Type
        compound_types::array_type();

        // Functions with Parameters
        functions::function_with_parameters(7);

        // Functions Multiple Parameters
        functions::function_multiple_parameters(7, 'h');

        // sum 2 numbers
        let result = functions::sum(3, 4);

        // display the result
        functions::display_result(result);

        // Function with Return Value
        let seven = functions::function_with_return_value();
        println!("The value of seven is: {seven}");

        // If Expression
        control_flow::if_expression();

        // If with Multiple Expressions
        control_flow::if_multiple_expressions();

        // Conditional Loops with While
        loops::conditional_loops_with_while();

        // Looping Through a Collection with For
        loops::looping_through_collection_with_for();

        // For Loop Elements
        loops::for_loop_elements();

        // Countdown Loop with Rev
        loops::countdown_loop_with_rev();

        // Slices
        slices::master();

        // Integers
        integers::master();
    }
}
