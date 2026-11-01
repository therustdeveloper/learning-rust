//! src/ownership/mod.rs
pub mod borrowing;
pub mod dereference_operator;
pub mod functions;
pub mod mutable_borrowing;
pub mod mutable_references;
pub mod ownership_arrays;
pub mod references_borrowing;
pub mod return_multiple_values;
pub mod return_values_and_scope;
pub mod returning_ownership;
pub mod string_clone_heap;
pub mod string_type;

crate::tui_module! {
    description: "Ownership, borrowing and references",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        // String Type
        string_type::index(false);

        // String Clone Heap
        string_clone_heap::index(false);

        // Moving a Value
        functions::index(false);

        // Return values and scope
        return_values_and_scope::index(false);

        // Return multiple values
        return_multiple_values::index(false);

        // References and Borrowing
        references_borrowing::index(false);

        // Mutable references
        mutable_references::index(false);

        // Borrowing
        borrowing::index(false);

        // Mutable Borrowing of Variables
        mutable_borrowing::index(false);

        // Returning Ownership
        returning_ownership::index(false);

        // Dereference Operator
        dereference_operator::index(false);

        // Ownership in Arrays
        ownership_arrays::index(false);
    }
}
