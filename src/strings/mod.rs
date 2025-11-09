//! src/strings/mod.rs
pub mod borrowed_string;
pub mod concatenation;
pub mod definition;
pub mod iterate;
pub mod matches;
pub mod string_functions;
pub mod string_is_encoded;
pub mod string_literals;

pub fn index(show: bool) {
    if show {
        // String Definition
        definition::index(false);

        // String Concatenation
        concatenation::index(false);

        // Methods for Iterating Over Strings
        iterate::index(false);

        // String Functions
        string_functions::index(false);

        // &str and String are encoded with UTF-8
        string_is_encoded::index(false);

        // String Literals
        string_literals::index(false);

        // Borrowed String
        borrowed_string::index(false);

        // String Matches
        matches::index(false);
    }
}
