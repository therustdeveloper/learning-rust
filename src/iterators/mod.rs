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

pub fn index(show: bool) {
    if show {
        println!("\n-- Iterators");

        definition::index(false);

        demo::master(false);

        activity::master(false);

        partition::master(false);

        iter_in_collections::master(false);

        extend::master(false);

        scan_hash_table::master(false);

        nonfunctional_style::master(false);

        functional_style::master(false);

        without_for::master(false);

        iterator_next::master(false);

        impl_iterator::master(false);

        mapping_and_filtering::master(false);

        filter_map::master(false);

        result_option_ok::master(false);

        rev::master(false);

        find_position::master(false);

        fold::master(false);

        skip::master(false);
    }
}
