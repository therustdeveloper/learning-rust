//! src/ranges/demo.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Range Demo");

        for num in 1..4 {
            println!("{:?}", num);
        }

        for ch in 'a'..='f' {
            println!("{:?}", ch);
        }

        // month_days is of type RangeInclusive<i32>
        let month_days = 1..=31;

        for day in month_days {
            println!("day: {}", day);
        }

        // grades is of type Range<i32>
        let grades = 1..6;

        for grade in grades {
            println!("grade: {}", grade);
        }

        let letters = 'a'..='z';

        for letter in letters {
            println!("{letter}");
        }

        let colors = ["Red", "Green", "Yello"];

        for color in colors {
            println!("{color}");
        }
    }
}
