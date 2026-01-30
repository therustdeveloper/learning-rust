use super::inventory;
use super::orders;
pub fn index(show: bool) {
    if show {
        println!("Inventory Program");
        inventory::talk_to_manager();
        println!(
            "Our managers are {} and {}. We have {} square feet of floor space",
            inventory::MANAGER,
            orders::MANAGER,
            inventory::FLOOR_SPACE
        );

        let macbook = inventory::Item {
            name: String::from("Extension Ladder"),
            category: inventory::ProductCategory::Ladder,
            quantity: 25,
        };

        let caldigit = inventory::Item {
            name: String::from("Claw Hammer"),
            category: inventory::ProductCategory::Hammer,
            quantity: 150,
        };

        println!(
            "Macbook: name = {}, category = {:?}, quantity = {}",
            macbook.name, macbook.category, macbook.quantity
        );
        println!(
            "Caldigit: name = {}, category = {:?}, quantity = {}",
            caldigit.name, caldigit.category, caldigit.quantity
        );
    }
}
