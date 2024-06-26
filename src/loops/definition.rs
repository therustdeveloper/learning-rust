pub fn master(show: bool) {
    if show {
        println!("-- Loops");

        loop_print_numbers_1_to_4();
    }
}

pub fn loop_print_numbers_1_to_4() {
    println!("\n--- Loop to print numbers from 1 to 4");
    let mut i = 1;

    loop {
        println!("The value of i: {}", i);
        if i == 4 {
            break;
        }
        i += 1;
    }
}
