use polars::error::PolarsResult;
use polars::prelude::{col, RollingOptionsFixedWindow};
use Backtester::data::data::DataHandler;
use Backtester::data::data::DataHandlerFetch;
use Backtester::strategy::strategy::Strategy;
use Backtester::strategy::strategy::StrategyTrait;

fn main() -> PolarsResult<()> {
    println!("Booting strategy!");
    let _symbol = "BTCUSDT";
    let _start_date = "2023-01-01";
    let _end_date = "2023-12-31";
    // TODO implement ticker! just a simple todo as if thats simple at all....

    let mut data = DataHandler::load_data("/Users/wexoah/RustroverProjects/Quant-trader/TA_Lib/examples/data/AMZN.csv");

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

    data = strategy.generate_signals(&mut &Some(data))?;

    println!("Indicators: {:?}", data);
    Ok(
        ()
    )
}
