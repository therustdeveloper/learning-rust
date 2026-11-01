use std::thread;

crate::tui_module! {
    description: "Zero-cost abstractions",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        let numbers = filter_even_numbers_new(vec![1, 2, 3, 4, 5, 6, 7]);
        println!("numbers: {:?}", numbers);

        // second zero cost abstraction example
        zca_threads();
    }
}

// example of zero cost abstraction 1
fn filter_even_numbers_new(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|&num| num % 2 == 0).collect()
}

// example of zero cost abstraction 2
fn zca_threads() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    let shared_data = std::sync::Arc::new(data);
    let handles: Vec<_> = (0..7)
        .map(|i| {
            let shared_data = shared_data.clone();
            thread::spawn(move || {
                let local_sum: i32 = shared_data.iter().sum();
                println!("Thread: {} Sum: {}", i, local_sum);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
