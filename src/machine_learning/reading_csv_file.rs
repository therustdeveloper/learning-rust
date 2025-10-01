//! src/machine_learning/reading_csv_file.rs
extern crate polars;
use polars::prelude::*;

pub fn index(show: bool) -> Result<(), PolarsError> {
    if show {
        println!("--- Reading CSV File\n");

        let options = CsvReadOptions::default()
            .with_has_header(true)
            .with_infer_schema_length(Some(100));

        let df = options
            .try_into_reader_with_file_path(Some(
                "./files/machine_learning/heart_attack_prediction_dataset.csv".into(),
            ))?
            .finish()?;

        println!("DataFrame: {:?}", df.shape());
    }

    Ok(())
}
