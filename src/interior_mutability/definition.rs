//! src/interior_mutability/definition.rs
use std::cell::Cell;

#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

impl PhoneModel {
    fn method_one(&self) {}
    fn method_two(&self) {}
    fn print(&self) {
        println!("Company Name: {:?}", self.company_name);
        println!("Model Name: {:?}", self.model_name);
        println!("Screen Size: {:?}", self.screen_size);
        println!("Memory: {:?}", self.memory);
        println!("Date Issued: {:?}", self.date_issued);
        println!("On Sale: {:?}", self.on_sale);
    }
    fn make_not_on_sale(&self) {
        self.on_sale.set(false);
    }
}

pub fn index(show: bool) {
    if show {
        println!("Interior Mutability Definition");

        let super_phone_3000 = PhoneModel {
            company_name: "YY Electronics".to_string(),
            model_name: "Super Phone 3000".to_string(),
            screen_size: 7.5,
            memory: 4_000_000,
            date_issued: 2020,
            on_sale: Cell::new(true),
        };

        super_phone_3000.method_one();
        super_phone_3000.method_two();
        super_phone_3000.print();

        println!("super_phone_3000: {:?}", super_phone_3000);

        super_phone_3000.make_not_on_sale();
        println!("super_phone_3000: {:?}", super_phone_3000);
    }
}
