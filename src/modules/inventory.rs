pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

#[derive(Debug)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

#[derive(Debug)]
pub struct Item {
    pub(crate) name: String,
    pub(crate) category: ProductCategory,
    pub(crate) quantity: u32,
}

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, how's your coffee?")
}
