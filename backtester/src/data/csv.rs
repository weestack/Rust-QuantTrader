use std::sync::Arc;
use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::{col, lit, CsvReadOptions, DataType, IntoLazy, TimeUnit};

pub fn load_csv(file_path: &str) -> DataFrame {
    // Open and read the CSV file
    let dtype_overwrite = Some(Arc::new(vec![
        DataType::String,
         DataType::Float64,
         DataType::Float64,
         DataType::Float64,
         DataType::Float64,
         DataType::Float64
    ]));


    let reader_options = CsvReadOptions {
        has_header: true,
        dtype_overwrite,
        infer_schema_length:Some(10000),
        //infer_schema_length: Some(1000000000),
        ..Default::default()
    };

    let mut df = reader_options
        .try_into_reader_with_file_path(Some(file_path.into()))
        .unwrap()
        .finish()
        .unwrap();

    let lowercased_cols: Vec<String> = df
        .get_column_names()
        .iter()
        .map(|name| name.to_lowercase())
        .collect();

    df.set_column_names(&lowercased_cols)
        .expect("Failed to set column names to lowercase");

    df = df
        .lazy()
        .with_columns([
            (col("timestamp").cast(DataType::Float64) * lit(1_000)) // Change from epoc in seconds to epoc in ms
                .cast(DataType::Datetime(TimeUnit::Milliseconds, None)),
        ]
        )
        .collect()
        .unwrap();

    df
}