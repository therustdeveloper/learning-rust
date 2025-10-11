//! src/traits/mod.rs
use crate::print_title;
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
pub mod returning_traits_with_dyn;
pub mod simple_trait;
pub mod with_generics;
mod verifying_with_traits;

pub fn index(show: bool) {
    if show {
        print_title("Traits");

        // Trait Demo
        demo::index(false);

        // Trait Activity
        activity::index(false);

        // Trait Definition
        definition::index(false);

        // Default Trait
        default_trait::index(false);

        // Trait using Derive Attribute
        derives::index(false);

        // Returning Traits with dyn
        returning_traits_with_dyn::index(false);

        // Media Aggregator
        media_aggregator::index(false);

        // Demo Trait Objects
        demo_trait_objets::index(false);

        // Activity Trait Objects
        activity_trait_objects::index(false);

        // From Trait
        from_trait::index(false);

        // With Generics
        with_generics::index(false);

        // Simple Trait Example
        simple_trait::index(false);

        // Implement Display Trait
        impl_display::index(false);

        // Implement Complex Trait
        complex_trait::index(false);

        // Copy Trait
        copy_trait::index(false);

        // Verifying with Traits
        verifying_with_traits::index(false);
    }
}
