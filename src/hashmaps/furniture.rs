//! src/hashmaps/furniture.rs
use crate::print_title;
use std::collections::HashMap;

pub fn index(show: bool) {
    if show {
        print_title("HashMap Furniture Activity");

        let mut stock = HashMap::new();

        stock.insert("Chair", 5);
        stock.insert("Bed", 3);
        stock.insert("Table", 2);
        stock.insert("Couch", 0);

        let mut total_stock = 0;

        for (item, qty) in stock.iter() {
            total_stock += qty;
            let stock_count = if qty == &0 {
                "out of stock".to_owned()
            } else {
                format!("{:?}", qty)
            };
            println!("item={:?}, stock={:?}", item, stock_count);
        }
        println!("total stock={:?}", total_stock);
    }
}
