//! src/basics/arrays.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("Arrays");

        demo(true);

        months(true);

        type_size(true);

        initial_values(true);

        basic_example(true);

        array_index_position(true);

        array_element_access(true);
    }
}

fn demo(show: bool) {
    if show {
        print_title("Array Demo");

        let a = [1, 2, 3, 4, 5, 6, 7];
        println!("a values: {:?}", a);
    }
}

fn months(show: bool) {
    if show {
        print_title("Months Array");

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
            "November",
            "December",
        ];

        println!("Months: {:?}", months);
    }
}

fn type_size(show: bool) {
    if show {
        print_title("Array Type & Size");

        let a: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
        println!("Values of a: {:?}", a);
    }
}

fn initial_values(show: bool) {
    if show {
        print_title("Array with initial values");

        let a = [7; 7];
        println!("Values of a: {:?}", a);
    }
}

fn basic_example(show: bool) {
    if show {
        let numbers = [4, 8, 15, 16, 23, 42];

        for number in numbers {
            println!("number: {number}");
        }

        println!("numbers lenght: {}", numbers.len());
    }
}

fn array_index_position(show: bool) {
    if show {
        let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

        println!("Spring: {}", seasons[0]);
        println!("Summer: {}", seasons[1]);
        println!("Fall: {}", seasons[2]);
        println!("Winter: {}", seasons[3]);

        seasons[2] = "Autumn";

        for season in seasons {
            println!("season: {}", season);
        }
    }
}

fn array_element_access(show: bool) {
    if show {
        println!("-- Array Element Access --");

        let a = [1, 2, 3, 4, 5];

        for i in a.iter() {
            println!("{i}");
        }
    }
}
