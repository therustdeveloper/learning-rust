//! src/generics/generic_functions.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        generic_functions(false);
    }
}

fn generic_functions(show: bool) {
    if show {
        print_title("Generic Functions");

        // Integers Vector
        let mut int_vector = vec![1, 2, 3, 4, 5, 6, 7];
        reverse(&mut int_vector);
        println!("int_vector after: {:?}", int_vector);

        // String Vector
        let mut str_vector = vec!["one", "two", "three", "four", "five", "six", "seven"];
        reverse(&mut str_vector);
        println!("str_vector after: {:?}", str_vector);

        // Bool Vector
        let mut bool_vector = vec![false, false, false, false, false, false, true];
        reverse(&mut bool_vector);
        println!("bool_vector after: {:?}", bool_vector);
    }
}

// remember that is a built-in method vector.reverse
fn reverse<T>(vector: &mut Vec<T>) {
    let mut new_vector = Vec::new();

    while let Some(last) = vector.pop() {
        new_vector.push(last);
    }

    *vector = new_vector;
}
