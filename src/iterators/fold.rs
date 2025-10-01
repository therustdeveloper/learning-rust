//! src/iterators/fold.rs
#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}

pub fn index(show: bool) {
    if show {
        fold_definition(false);

        fold_to_combine(false);
    }
}

#[allow(clippy::unnecessary_fold)]
fn fold_definition(show: bool) {
    if show {
        println!("---Fold definition");

        let some_numbers = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        println!(
            "{}",
            some_numbers
                .iter()
                .fold(0, |total_so_far, next_number| total_so_far + next_number)
        );
    }
}

fn fold_to_combine(show: bool) {
    if show {
        println!("---Fold to combine");

        let events = [
            "Went to grocery store",
            "Came home",
            "Fed cat",
            "Fed cat again",
        ];

        let empty_events = CombinedEvents {
            num_of_events: 0,
            data: vec![],
        };

        let combined_events = events
            .iter()
            .fold(empty_events, |mut total_events, next_event| {
                total_events.num_of_events += 1;
                total_events.data.push(next_event.to_string());
                total_events
            });

        println!("{combined_events:#?}");
    }
}
