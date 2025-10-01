//! src/traits/demo.rs
trait Fall {
    fn hit_ground(&self);
}

struct Vase;

impl Fall for Vase {
    fn hit_ground(&self) {
        println!("the vase broke!");
    }
}

struct Cat;

impl Fall for Cat {
    fn hit_ground(&self) {
        println!("the cat casually walked away")
    }
}

/// fall function receives an argument that implements Fall trait
fn fall(thing: impl Fall) {
    thing.hit_ground();
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Trait Demo");

        fall(Vase {});
        fall(Cat {});
    }
}
