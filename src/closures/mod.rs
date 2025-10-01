//! src/closures/mod.rs
pub mod closures_inside_methods;
pub mod definition;
pub mod demo;
pub mod map_collect;
pub mod map_combinator;
pub mod process_closure_thread;

pub fn index(show: bool) {
    if show {
        println!("\n-- Closures");

        definition::index(false);

        process_closure_thread::index(false);

        demo::index(false);

        map_combinator::index(true);

        closures_inside_methods::index(false);

        map_collect::index(false);
    }
}
