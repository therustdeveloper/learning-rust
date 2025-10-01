//! src/hashmaps/mod.rs
pub mod activity;
pub mod definition;
pub mod furniture;
pub mod hashmap_entry_method;
pub mod hashmap_get;
pub mod hashmap_loop;
pub mod hashmap_old_values;

pub fn index(show: bool) {
    if show {
        // HashMap Definition
        definition::index(false);

        // HashMap Activity
        activity::index(false);

        // HashMap Furniture Activity
        furniture::index(false);

        // HashMap in for
        hashmap_loop::index(false);

        // HashMap Get Method
        hashmap_get::index(false);

        // HashMap Old Values
        hashmap_old_values::index(false);

        // HashMap Entry Method
        hashmap_entry_method::master(false);
    }
}
