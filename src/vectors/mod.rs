//! src/vectors/mod.rs
pub mod add_remove;
pub mod create_vector;
pub mod definition;
pub mod get_method;
pub mod reading_vector;
pub mod sort_vector;
pub mod vector_capacity;

pub fn index(show: bool) {
    if show {
        println!("\n-- Vectors");

        definition::index(true);

        create_vector::index(false);

        sort_vector::index(false);

        add_remove::index(false);

        reading_vector::index(false);

        get_method::index(false);

        vector_capacity::index(false);
    }
}
