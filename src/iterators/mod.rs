//! src/iterators/mod.rs
pub mod activity;
pub mod definition;
pub mod demo;
pub mod extend;
pub mod filter_map;
pub mod find_position;
pub mod fold;
pub mod functional_style;
pub mod impl_iterator;
pub mod iter_in_collections;
pub mod iterator_next;
pub mod mapping_and_filtering;
pub mod nonfunctional_style;
pub mod partition;
pub mod result_option_ok;
pub mod rev;
pub mod scan_hash_table;
pub mod skip;
pub mod without_for;

crate::tui_module! {
    description: "Iterators, adapters and functional style",
    category: Intermediate,
    sections_only,
}
