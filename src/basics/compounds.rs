//! src/basics/compounds.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("Compound Types");

        tuple_type(true);

        array_type(true);
    }
}

fn tuple_type(show: bool) {
    if show {
        println!("--- Tuple Type ---");
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        println!("The value of tup is: {:?}", tup);

        let (x, y, z) = tup;
        println!("The value of x is: {x}");
        println!("The value of y is: {y}");
        println!("The value of z is: {z}");

        println!("The first value of the tuple is: {:?}", tup.0);
        println!("The second value of the tuple is: {:?}", tup.1);
        println!("The third value of the tuple is: {:?}", tup.2);
    }
}

fn array_type(show: bool) {
    if show {
        println!("--- Array Type ---");
        let a = [1, 2, 3, 4, 5, 6, 7];
        println!("The value of array a is: {:?}", a);

        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "December",
        ];
        println!("The value of months is: {:?}", months);

        let b: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
        println!("The value of b array is: {:?}", b);

        let c = [7; 7];
        println!("The value of c array is: {:?}", c);

        let first = b[0];
        println!("The value of first is: {first}");

        let second = b[1];
        println!("The value of second is: {second}");
    }
}
