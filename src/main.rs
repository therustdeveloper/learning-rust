pub mod structs;
pub mod random_numbers;
pub mod errors;
pub mod traits;
pub mod get_type;

//use std::{cmp::Ordering, io};

fn main() {
    // Basics
    common::basics::master(false);

    // Local Function
    local_function(false);

    // Ownership
    common::ownership::master(false);

    // Structs
    structs::master(false);

    // Methods
    common::methods::master(false);

    // Enums
    common::enums::master(false);

    // Random Numbers
    random_numbers::master(false);

    // Algorithms
    common::algorithms::master(false);

    // Vectors
    common::collections::master(false);

    // HashMaps
    common::hashmaps::master(false);

    // Exercises
    common::exercises::master(false);

    // Errors
    errors::master(false);

    // Traits
    traits::master(false);

    // Lifetimes
    common::lifetimes::master(false);

    // Strings
    common::strings::master(false);

    // Generics
    common::generics::master(false);

    // Tests
    common::tests::definition::master(false);

    // Closures
    common::closures::master(false);

    // Iterators
    common::iterators::master(false);

    // Doc
    common::doc::master(false);

    // Boxes
    common::boxes::master(false);

    // Custom Smart Pointer
    common::custom_smart_pointer::master(false);

    // Rc<T>
    common::rc_t::definition::master(false);

    // Messenger Application
    common::messenger::master(false);

    // Tree Data Structure
    common::tree_data_structure::definition::master(false);

    // Concurrency
    common::threads::definition::master(false);

    // JoinHandle
    common::joinhandle::master(false);

    // Message Passing
    common::message_passing::master(false);

    // The API of Mutex<T>
    common::api_mutex::master(false);

    // Object Oriented
    common::object_oriented::master(false);

    // Draw Trait
    common::draw::master(false);

    // Blog Post
    common::post::master(false);

    // Patterns
    common::patterns::master(false);

    // Destructuring Structs
    common::destructuring_structs::master(false);

    // Unsafe Code
    common::unsafe_code::definition::master(false);

    // Using Extern Functions
    common::extern_functions::master(false);

    // Accessing or Modifying a Mutable Static Variable
    common::static_variable::definition::master(false);

    // Implementing an Unsafe Trait
    common::unsafe_trait::definition::master(false);

    // Operator Overloading
    common::operator_overloading::master(false);

    // Implementing the Add trait on Millimeters to add Millimeters and Meters
    common::add_millimeters_to_meters::master(false);

    // Vectors
    common::vectors::master(false);

    // Futures
    common::futures::master(false);

    // Get Type
    get_type::master(false);

    // Mutable Reference
    common::mutable_reference::master(false);

    // Making Decisions with Rust
    common::decisions::master(false);

    // Match Expression
    common::match_expression::master(false);

    // Loops
    common::loops::master(false);

    // While Loops
    common::while_loop::definition::master(false);

    // Tuples
    common::tuples::master(false);

    // Expressions
    common::expressions::master(false);

    // Arrays
    common::arrays::master(false);

    // Print variable type
    common::print_var_type::master(false);

    // Move vars
    common::move_vars::master(false);

    // Data Collections
    common::data_collections::master(false);

    // References
    common::references::master(false);

    // Advanced Match
    common::advanced_match::master(false);

    // Optionals
    common::optionals::master(false);

    // Results
    common::results::master(false);
}

fn local_function(show: bool) {
    if show {
        println!("--- Local Function ---");
        println!("Hello from local function!");
    }
}
