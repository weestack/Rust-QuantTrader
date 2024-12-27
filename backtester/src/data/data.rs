use polars::prelude::DataFrame;
use crate::data::csv::load_csv;

pub trait DataHandlerFetch<T> where T: Into<&'static str> {
    fn load_data(options: T) -> DataFrame;
}

#[allow(dead_code)]
pub struct DataHandler {
    symbol: String,
    start_date: String,
    end_date: String,
}

impl<T> DataHandlerFetch<T> for DataHandler
where
    T: Into<&'static str>,
{
    fn load_data(options: T) -> DataFrame {
        let filename: &str = options.into();
        load_csv(filename) // This assumes `load_csv` is defined and returns `DataFrame`.
    }
}