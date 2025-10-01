//! src/data_collections/mod.rs
pub mod definition;
pub mod shipping_box;

pub fn index(show: bool) {
    if show {
        println!("\n-- Data Collections");

        definition::index(false);

        shipping_box::index(false);
    }
}
