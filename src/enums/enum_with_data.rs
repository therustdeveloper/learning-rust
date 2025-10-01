//! src/enums/enum_with_data.rs
use crate::print_title;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

// Enums can have methods
impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

/// enum inside of enum
/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" insted of "February 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

pub fn index(show: bool) {
    if show {
        print_title("Enums with Data");

        let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
        println!(
            "{:?}",
            rough_time_to_english(four_score_and_seven_years_ago)
        );

        let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
        println!("{:?}", rough_time_to_english(three_hours_from_now));

        let seven_seconds_from_now = RoughTime::InTheFuture(TimeUnit::Seconds, 7);
        println!("{:?}", rough_time_to_english(seven_seconds_from_now));

        let seven_minutes_from_now = RoughTime::InTheFuture(TimeUnit::Minutes, 7);
        println!("{:?}", rough_time_to_english(seven_minutes_from_now));

        let seven_months_from_now = RoughTime::InTheFuture(TimeUnit::Months, 7);
        println!("{:?}", rough_time_to_english(seven_months_from_now));

        let seven_days_from_now = RoughTime::InTheFuture(TimeUnit::Days, 7);
        println!("{:?}", rough_time_to_english(seven_days_from_now));

        let from_now = RoughTime::JustNow;
        println!("{:?}", rough_time_to_english(from_now));

        println!("{:?}", TimeUnit::Days.singular());
    }
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("{:?}", "Just Now".to_string()),
        RoughTime::InTheFuture(unit, 1) => format!("a {} from now", unit.singular()),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}
