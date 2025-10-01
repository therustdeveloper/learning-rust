//! src/iterators/definition.rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

pub fn index(show: bool) {
    if show {
        println!("--- ITERATORS ---");
        creating_an_iterator();

        filter_by_size();
    }
}

fn creating_an_iterator() {
    println!("\n-- Creating and Iterator");
    //let v1 = vec![1, 2, 3];
    let v1_iter = [1, 2, 3].iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filter_by_size() {
    println!("\n- Using the filter method with a closure");
    let other_shoes = vec![
        Shoe {
            size: 11,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 14,
            style: String::from("sandal"),
        },
        Shoe {
            size: 12,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(other_shoes, 10);

    println!("value of in_my_size: {:?}", in_my_size);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
