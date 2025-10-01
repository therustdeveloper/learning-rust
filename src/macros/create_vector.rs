//! src/macros/create_vector.rs
// declarative macros
macro_rules! my_vector {
    () => { // first matcher
        Vec::new()
    };
    (make an empty vector) => ( // second matcher
        Vec::new()
    );
    {$x:expr} => {
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };
    [$($x:expr),+] => (
        {
            let v = vec![..];
            v
        }
    )
}

pub fn index(show: bool) {
    if show {
        println!("---Create Vector");

        let empty: Vec<i32> = my_vector![];
        println!("{:?}", empty);

        let also_empty: Vec<i32> = my_vector!(make an empty vector);
        println!("{:?}", also_empty);

        let three_numbers = my_vector!(1, 2, 3);
        println!("{:?}", three_numbers);
    }
}
