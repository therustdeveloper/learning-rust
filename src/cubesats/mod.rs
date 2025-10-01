//! src/cubesats/mod.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("CubeSats");

        let sat_a = CubeSat { id: 0 };
        let sat_b = CubeSat { id: 0 };
        let sat_c = CubeSat { id: 0 };

        print_status(&sat_a, &sat_b, &sat_c);

        print_status(&sat_a, &sat_b, &sat_c);
    }
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(_sat_id: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn print_status(sat_a: &CubeSat, sat_b: &CubeSat, sat_c: &CubeSat) {
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a status: {:?}, a id: {:?}", a_status, sat_a.id);
    println!("b status: {:?}, b id: {:?}", b_status, sat_b.id);
    println!("c status: {:?}, c id: {:?}", c_status, sat_c.id);
}
