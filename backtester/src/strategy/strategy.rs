use polars::prelude::*;
use std::sync::Arc;

// Define a type for an indicator function.
// It takes and modifies a `DataFrame` (e.g., adding a new column).
pub type IndicatorFn = Arc<dyn Fn(&mut DataFrame) -> PolarsResult<()>>;

pub trait StrategyTrait {
    fn generate_signals(&self, data: &mut &Option<DataFrame>) -> PolarsResult<DataFrame>;
    fn apply_strategy(&self, df: &mut &Option<DataFrame>) -> PolarsResult<DataFrame>;
}


// Struct to represent the trading strategy.
#[derive(Debug)]
pub struct Strategy<E: AsRef<[Expr]>, T: AsRef<[Expr]>> {
    indicators: E,
    signal_logic: T
}
impl<E: AsRef<[Expr]>, T: AsRef<[Expr]>> Strategy<E, T> {
    pub fn new(
        indicators: E,
        signal_logic: T
    ) -> Self {
        Self {
            indicators,
            signal_logic,
        }
    }
}

impl<E: AsRef<[Expr]>, T: AsRef<[Expr]>> StrategyTrait for Strategy<E, T> {
    /// Create a new strategy with indicators and signal logic.
    /// Generate trading signals for the given dataset (single or multiple assets).
    fn generate_signals(&self, data: &mut &Option<DataFrame>) -> PolarsResult<DataFrame> {
        let indicators = self.apply_strategy(data)?;
        let signals = indicators
            .clone()
            .lazy()
            .with_columns(self.signal_logic.as_ref())
            .select([col("timestamp"), col("signal"), col("close")])
            .collect()?;
        Ok(signals)
    }

    /// Apply the entire strategy (indicators and signal logic) to the DataFrame.
    fn apply_strategy(&self, df: &mut &Option<DataFrame>) -> PolarsResult<DataFrame> {
        // Apply all indicators to the DataFrame, adding new columns
        Ok(df.clone().unwrap().lazy().with_columns(self.indicators.as_ref()).collect()?)
    }
}