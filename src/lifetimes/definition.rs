//! src/lifetimes/definition.rs
#[derive(Debug)]
enum FrozenItem {
    IceCube,
}

#[derive(Debug)]
struct Freezer {
    contents: Vec<FrozenItem>,
}

pub fn index(show: bool) {
    if show {
        println!("-- Lifetimes Definition");

        let result = longest("one", "two");
        println!("The result is {}", result);

        let mut freezer = Freezer { contents: vec![] };
        let cube = FrozenItem::IceCube;
        place_item(&mut freezer, cube);
        // cube no longer available
        println!("Freezer: {:?}", freezer);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn place_item(freezer: &mut Freezer, item: FrozenItem) {
    freezer.contents.push(item);
}
