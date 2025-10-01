//! src/unsafe_code/definition.rs
use std::slice;

pub fn index(show: bool) {
    if show {
        println!("-- Unsafe Code");

        dereferencing_raw_pointer();
        calling_unsafe_function_or_method();
        creating_safe_abstraction_over_unsafe_code();
    }
}

fn dereferencing_raw_pointer() {
    println!("-- Dereferencing a Raw Pointer");

    let mut num = 5;

    // Creating Raw Pointers
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferencing raw pointers within an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn calling_unsafe_function_or_method() {
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

fn creating_safe_abstraction_over_unsafe_code() {
    let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let (vec1, vec2) = split_at_mut(&mut array, 7);

    println!("vec1 values: {:?}", vec1);
    println!("vec2 values: {:?}", vec2);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
