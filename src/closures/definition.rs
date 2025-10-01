//! src/closures/definition.rs
use std::thread;

pub fn index(show: bool) {
    if show {
        println!("--- CLOSURES ---");
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );

        // closure immutable reference
        closure_immutable_reference();

        // closure mutable reference
        closure_mutable_reference();

        // closure thread to take ownership
        closure_thread_to_take_ownership();

        // sorting a struct
        sorting_a_struct();
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn closure_immutable_reference() {
    println!("\n-- Closure Immutable Reference");
    let list = vec![0, 1, 1, 2, 3, 5, 8];
    println!("Before defining closure: {:?}", list);

    // this is a closure definition
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);

    // calling the closure
    only_borrows();

    // because we can have multiple immutable references to list at the same time
    println!("After calling closure: {:?}", list);
}

fn closure_mutable_reference() {
    println!("\n-- Closure Mutable Reference");
    let mut list = vec![0, 1, 1, 2, 3, 5, 8];
    println!("Before defining closure: {:?}", list);

    // closure definition
    let mut borrows_mutably = || list.push(13);

    // note that there's no longer a println! between the definition
    // and the call of the borrows_mutably closure:
    // when borrows_mutably is defined, it captures a mutable reference to list

    // calling the closure
    borrows_mutably();

    println!("After calling closure: {:?}", list);
}

fn closure_thread_to_take_ownership() {
    println!("\n-- Closure Thread to Take Ownership");
    let mut list = vec![0, 1, 1, 2, 3, 5, 8];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || {
        list.push(13);
        println!("From thread: {:?}", list)
    })
    .join()
    .unwrap();

    // cannot use the list variable after calling the closure
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sorting_a_struct() {
    println!("\n-- Sorting a struct");
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 17,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    list.sort_by_key(|r| r.height);
    println!("{:#?}", list);
}
