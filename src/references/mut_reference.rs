//! src/references/mut_reference.rs
pub fn index(show: bool) {
    if show {
        println!("--- Mutable Reference");

        let mut my_number = 8;
        let num_ref = &mut my_number;

        *num_ref += 1;
        println!("my_number: {}", my_number);

        let second_number = 800;
        let triple_reference = &&&second_number;
        println!("Are they equal? {}", second_number == ***triple_reference);

        mutable_borrow_together(false);
    }
}

pub fn mutable_borrow_together(show: bool) {
    if show {
        println!("--- Mutable Borrow Together");

        let mut number = 10;
        let number_change = &mut number;
        *number_change += 10; // the mutable action finished here

        let number_ref = &number; // other reference starts here
        println!("number_ref: {}", number_ref);
    }
}
