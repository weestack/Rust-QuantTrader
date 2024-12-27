#[cfg(test)]
mod tests {
    use polars::error::PolarsResult;
    use polars::prelude::{col, RollingOptionsFixedWindow};
    use Backtester::backtrader::backtrader::Backtrader;
    use Backtester::strategy::strategy::Strategy;
    use super::*;

    #[test]
    fn test_strategy_with_backtrader() -> PolarsResult<()> {
        println!("Booting strategy!");
        use std::time::Instant;

        let before = Instant::now();
        let symbol = "BTCUSDT";
        let start_date = "2023-01-01";
        let end_date = "2023-12-31";
        // TODO implement ticker! just a simple todo as if thats simple at all....

        //let mut data = DataHandler::load_data("backtester/examples/data/btcusd_1-min_data.csv");

        let window_20 = RollingOptionsFixedWindow {
            window_size: 3,
            ..RollingOptionsFixedWindow::default()
        };

        let window_60 = RollingOptionsFixedWindow {
            window_size: 8,
            ..RollingOptionsFixedWindow::default()
        };

        let indicator_expr = [
            col("close").rolling_mean(window_20).alias("sma_20"),
            col("close").rolling_mean(window_60).alias("sma_60")
        ];

        let signal_expr = [
            col("sma_20").gt(col("sma_60")).alias("signal")
        ];

        let strategy = Strategy::new(
            indicator_expr,
            signal_expr
        );

        // Shape strategy
        // pass strategy to backtrader
        // let backtrader run it candle by candle
        // let backtrader fetch the data based on settings and perhaps chosen exchange?
        // get performance from backtrader

        //data = strategy.generate_signals(&mut data)?;

        //let mut backtrader = Backtrader::new(1000.0, 0.001, 1.0);
        //backtrader.backtest(data);
        //backtrader.calculate_performance(false);

        //println!("Indicators: {:?}", data);
        let mut backtrader = Backtrader::new(
            1000.0,
            0.001,
            1.0,
            vec![&"BTCUSDT".to_string()],
        );
        backtrader.backtest(Some("BTCUSDT".to_string()), strategy).unwrap();

        println!("{:?}", backtrader);

        println!("Elapsed time: {:.2?}", before.elapsed());
        Ok(
            ()
        )
    }


    #[test]
    fn data_source_present() {
        let data_source = "examples/data/btcusd_1-min_data.csv";
        if !std::path::Path::new(data_source).exists() {
            assert!(false, "Data source not found, perhaps you forgot to run fetch_test_data.sh?");
        } else {
            assert!(true);
        }
    }
}