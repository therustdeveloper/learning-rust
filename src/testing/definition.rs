//! src/testing/definition.rs
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {name}")
}

pub fn index(show: bool) {
    if show {
        let large_rect = Rectangle {
            width: 10,
            height: 10,
        };

        let small_rect = Rectangle {
            width: 7,
            height: 7,
        };

        let result = large_rect.can_hold(&small_rect);

        println!("Result: {}", result);

        let value = add_two(5);

        println!("Value is: {}", value);

        let name = greeting("William");

        println!("Name: {name}");
    }
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn that_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };

    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 7,
        height: 7,
    };

    let smaller = Rectangle {
        width: 3,
        height: 3,
    };

    assert!(!smaller.can_hold(&larger));
}

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was`{result}`"
    );
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&5));
    assert_eq!(v1_iter.next(), Some(&6));
    assert_eq!(v1_iter.next(), Some(&7));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 28);
}

#[test]
fn produce_other_iterators() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4, 5, 6, 7, 8]);
}
