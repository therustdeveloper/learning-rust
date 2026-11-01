//! src/hashmaps/mod.rs
pub mod activity;
pub mod character_profile;
pub mod definition;
pub mod furniture;
pub mod hashmap_entry_method;
pub mod hashmap_get;
pub mod hashmap_loop;
pub mod hashmap_old_values;

crate::tui_module! {
    description: "Storing key-value pairs with HashMap",
    category: DataStructures,
    sections_only,
}
