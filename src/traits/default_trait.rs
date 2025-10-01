//! src/traits/default_trait.rs
use crate::print_title;

struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Package {
        Self { weight }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}

pub fn index(show: bool) {
    if show {
        print_title("Default Trait");

        let p0 = Package::new(7.0);
        println!("Package Weight: {:?}", p0.weight);

        let p1 = Package::default();
        println!("Package Weight: {:?}", p1.weight);
    }
}
