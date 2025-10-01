//! src/iterators/mapping_and_filtering.rs
pub fn index(show: bool) {
    if show {
        println!("--- Mapping and Filtering");

        filtering(false);
    }
}

fn filtering(show: bool) {
    if show {
        let months = vec![
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        let filtered_months = months
            .into_iter()
            .filter(|month| month.len() < 5)
            .filter(|month| month.contains("u"))
            .collect::<Vec<&str>>();

        println!("{:?}", filtered_months);
    }
}
