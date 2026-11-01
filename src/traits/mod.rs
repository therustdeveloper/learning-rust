//! src/traits/mod.rs
pub mod activity;
pub mod activity_trait_objects;
pub mod complex_trait;
pub mod copy_trait;
pub mod default_trait;
pub mod definition;
pub mod demo;
pub mod demo_trait_objets;
pub mod derives;
pub mod from_trait;
pub mod impl_display;
pub mod media_aggregator;
pub mod printable;
pub mod returning_traits_with_dyn;
pub mod simple_trait;
pub mod verifying_with_traits;
pub mod with_generics;

crate::tui_module! {
    description: "Traits, trait objects and derives",
    category: Intermediate,
    sections_only,
}
