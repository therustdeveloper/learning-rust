//! src/vectors/definition.rs
pub fn index(show: bool) {
    if show {
        definition(false);

        build_vector_from_iterator(false);

        reverse_vector(false);

        vector_capacity(false);

        vector_pop(false);

        vector_iterate(false);

        vector_is_empty(false);
    }
}

fn definition(show: bool) {
    if show {
        println!("\n--- Vector Definition");

        let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
        println!("Initial value of string_vector: {:?}", string_vector);

        // add a value to string_vector
        string_vector.push("four");
        println!("Append a value to string_vector: {:?}", string_vector);

        // vector sorting
        string_vector.sort();
        println!("string_vector sorted: {:?}", string_vector);

        // use a function to build a vector
        let v = build_vector();
        println!("Value of v: {:?}", v);
    }
}

// type inference
fn build_vector() -> Vec<i16> {
    vec![7, 77]
    /*let mut v = Vec::new();
    v.push(7);
    v.push(77);
    v*/
}

// build a vector from the values produced by an iterator
fn build_vector_from_iterator(show: bool) {
    if show {
        println!("\n--- build a vector from the values produced by an iterator");
        let v: Vec<i32> = (0..7).collect();
        println!("v: {:?}", v);
    }
}

fn reverse_vector(show: bool) {
    if show {
        println!("\n--- reverse a vector");

        let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
        println!("palindrome: {:?}", palindrome);
        palindrome.reverse();
        println!("palindrome reversed: {:?}", palindrome);
    }
}

fn vector_capacity(show: bool) {
    if show {
        println!("\n--- vector capacity");

        let mut v: Vec<i32> = Vec::with_capacity(2);
        println!("vector v initial len: {}", v.len());
        println!("vector v initial capacity: {}", v.capacity());

        v.push(1);
        v.push(2);
        println!("vector v len: {}", v.len());
        println!("vector v capacity: {}", v.capacity());

        v.push(3);
        println!("vector v len: {}", v.len());
        println!("vector v capacity: {}", v.capacity());

        v.remove(2);
        println!("vector v len: {}", v.len());
        println!("vector v capacity: {}", v.capacity());

        v.push(3);
        println!("vector v len: {}", v.len());
        println!("vector v capacity: {}", v.capacity());
    }
}

fn vector_pop(show: bool) {
    if show {
        println!("--- vector pop");

        let mut v = vec!["Snow Puff", "Glass Gem"];
        println!("vector v initial values: {:?}", v);

        let mut item = v.pop().unwrap_or("").to_string();
        println!("vector v values: {:?}", v);
        println!("value returned: {}", item);

        item = v.pop().unwrap_or("").to_string();
        println!("vector v values: {:?}", v);
        println!("value returned: {}", item);

        item = v.pop().unwrap_or("").to_string();
        println!("vector v values: {:?}", v);
        println!("value returned: {}", item);
    }
}

fn vector_iterate(show: bool) {
    if show {
        println!("--- Vector iterate");

        //let languages = vec!["c", "go", "rust"];
        for (index, l) in ["c", "go", "rust"].iter().enumerate() {
            println!("language[{}]: {}", index, l);
        }
    }
}

fn vector_is_empty(show: bool) {
    if show {
        println!("\n--- Vector is empty");

        //let numbers = vec![1, 2, 3];
        #[allow(clippy::const_is_empty)]
        match [1, 2, 3].is_empty() {
            true => println!("numbers vector has no numbers"),
            false => println!("numbers vector has numbers"),
        }
    }
}
