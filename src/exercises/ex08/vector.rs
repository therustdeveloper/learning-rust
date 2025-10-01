//! src/exercises/ex08/vector.rs
use std::collections::HashMap;

pub fn index() {
    println!("--- Vector and show the average, median and mode ---");

    let mut v = vec![
        17, 7, 27, 47, 37, 67, 57, 87, 77, 97, 107, 117, 127, 137, 147, 197, 187, 177, 167, 157,
    ];
    println!("Original value of v: {:?}", v);

    println!("Average: {}", average(&v));
    println!("Median: {}", median(&mut v));
    println!("Mode: {}", mode(&v));
}

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}
