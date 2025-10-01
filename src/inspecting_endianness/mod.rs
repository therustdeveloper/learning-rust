//! src/inspecting_endianness/mod.rs
use crate::print_title;

pub fn index(show: bool) {
    if show {
        print_title("Inspecting Endianness");

        let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];

        let a: i32 = i32::from_ne_bytes(big_endian);
        let b: i32 = i32::from_ne_bytes(little_endian);

        println!("{} vs  {}", a, b);
    }
}
