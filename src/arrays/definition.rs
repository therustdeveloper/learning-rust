//! src/arrays/definition.rs
pub fn index(show: bool) {
    if show {
        definition(false);

        long_array(false);

        sorting_array(true);
    }
}

fn definition(show: bool) {
    if show {
        println!("\n--- Array Definition");

        let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
        let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

        // Some Prints
        println!("lazy_caterer: {:?}", lazy_caterer);
        println!("taxonomy: {:?}", taxonomy);
    }
}

fn long_array(show: bool) {
    if show {
        println!("--- Long Array");

        let mut sieve = [true; 10000];
        for i in 2..100 {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }

        println!("sieve[777]: {:?}", sieve[777]);
        println!("!sieve[7777]: {:?}", !sieve[7777]);
    }
}

fn sorting_array(show: bool) {
    if show {
        println!("--- Sorting an Array");

        let mut chaos = [3, 5, 4, 1, 2, 7, 6];
        println!("chaos initial value: {:?}", chaos);

        // sorting chaos
        chaos.sort();
        println!("chaos new value: {:?}", chaos);
    }
}
