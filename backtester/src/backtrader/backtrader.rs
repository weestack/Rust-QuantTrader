use polars::prelude::*;
use std::collections::HashMap;
use crate::backtrader::asset_data::AssetData;
use crate::backtrader::exchange::Exchange;
use crate::strategy::strategy::{Strategy, StrategyTrait};

pub type PortfolioHistory = HashMap<String, Vec<f64>>;
pub type DailyPortfolioValues = Vec<f64>;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Backtrader {
    initial_capital: f64,
    exchange: Exchange,
    assets_data: HashMap<String, AssetData>, // Asset data keyed by asset symbol.
    portfolio_history: PortfolioHistory,    // Historical total values of all assets.
    daily_portfolio_values: DailyPortfolioValues, // Total portfolio value over time.
}



impl Backtrader {
    // No self parameter here, as new creates a new instance
    pub fn new(initial_capital: f64, commission_pct: f64, commission_fixed: f64, symbols: Vec<&String>) -> Self {
        let mut assets_data: HashMap<String, AssetData> = HashMap::new();
        let symbol_capital = initial_capital / symbols.len() as f64;
        for symbol in symbols {
            let asset_data = AssetData::new(symbol.as_str(), symbol_capital, 0.0, 0.0);
            assets_data.insert(symbol.clone(), asset_data.clone());
        }

        /* TODO Implement changing Exchange and make data fetching exchange dependant */

        Self {
            initial_capital,
            exchange: Exchange {
                name: "Binance".to_string(),
                commission_pct,
                commission_fixed,
            },
            assets_data: assets_data.clone(),
            portfolio_history: HashMap::new(),
            daily_portfolio_values: Vec::new(),
        }
    }



    // Takes &mut self since this will modify the Backtrader instance by executing a trade
    #[inline(always)]
    fn execute_trade(&mut self, symbol: &str, signal: i32, price: f64) {
        // Retrieve the asset data for the symbol
        if let Some(asset) = self.assets_data.get_mut(symbol) {
            {
                if signal > 0 && asset.cash > 0.0 { // Buy signal logic
                    let trade_value = asset.cash;
                    let commission = self.exchange.calculate_commission(trade_value);
                    let shares_to_buy = (trade_value - commission) / price;
                    asset.positions += shares_to_buy;
                    asset.cash -= trade_value;
                } else if signal < 0 && asset.positions > 0.0 { // Sell signal logic
                    let trade_value = asset.positions * price;
                    let commission = self.exchange.calculate_commission(trade_value);
                    asset.cash += trade_value - commission;
                    asset.positions = 0.0;
                }

                // Update position value and total value
                asset.position_value = asset.positions * price;
                asset.total_value = asset.cash + asset.position_value;

                // Update total value history for tracking
                asset.history.push(asset.total_value);
            }
        } else {
            eprintln!("Asset '{}' not found in portfolio.", symbol);
        }
    }

    // Takes &mut self since it likely updates the portfolio
    fn update_portfolio(&mut self, symbol: &str, price: f64) {
        // Retrieve the asset data for the symbol
        if let Some(asset) = self.assets_data.get_mut(symbol) {
            // Update position value (positions * latest price)
            asset.position_value = asset.positions * price;

            // Update total value (cash + position value)
            asset.total_value = asset.cash + asset.position_value;

            // Add the updated total value to history for the specific asset
            asset.history.push(asset.total_value);

            // Update the overall portfolio's daily value (optional)
            if let Some(last_value) = self.daily_portfolio_values.last_mut() {
                *last_value += asset.total_value;
            } else {
                self.daily_portfolio_values.push(asset.total_value);
            }
        } else {
            eprintln!("Asset '{}' not found in portfolio, cannot update portfolio.", symbol);
        }
    }

    // Takes &mut self since it likely modifies or interacts with the Backtrader instance during the backtest process
    pub fn backtest(&mut self, symbol: Option<String>, strategy: impl StrategyTrait) -> Result<(), PolarsError> {
        // Split initial capital equally among all assets in data
        let assets_count = self.assets_data.len() as f64;
        let initial_allocation = self.initial_capital / assets_count;

        let symbols = if symbol.is_some() {
            vec![symbol.unwrap()]
        } else {
            self.assets_data.keys().cloned().collect()
        };

        for symbol in symbols {
            println!("Backtesting asset: {}", symbol);
            let asset = self.assets_data.get_mut(&symbol).unwrap();
            asset.load_data();

            let mut signals = strategy.generate_signals(&mut asset.get_data())?;
            let expr = col("signal").eq(true);
            let final_signals = signals.lazy().with_columns([expr]).collect()?;
            /* Rather naive, move some of the logic to strategy for flexibility TODO */
            for i in 0..final_signals.height() {
                let price = final_signals.column("close")?.get(i)?.try_extract::<f64>()?;
                let signal = final_signals.column("signal")?.get(i)?.try_extract::<i32>()?;
                /* TODO make consecutive aware so multiple true in a row does not make it fire the entire cash holdings within n consecutive true signals */
                let timestamp = final_signals.column("timestamp")?.get(i);

                self.execute_trade(&symbol, signal, price);
                self.update_portfolio(&symbol, price);
            }
        }

        self.daily_portfolio_values = self.assets_data.values()
        .map(|asset| asset.history.clone())
        .fold(vec![0.0; assets_count as usize], |mut acc, hist| {
            for (i, value) in hist.iter().enumerate() {
                acc[i] += value;
            }
            acc
        });

        Ok(())
    }

    // Takes &self since performance calculation likely doesn't modify the Backtrader instance
    #[allow(dead_code)]
    pub fn calculate_performance(&self, _plot: bool /* TODO implement plotting */ ) -> Result<(), PolarsError> {
        if self.daily_portfolio_values.is_empty() {
            println!("No daily portfolio values found nothing to calculate performance over.");
            return Ok(())
        }
        // Add function name for easier debugging
        unimplemented!("Function 'calculate_performance' is not implemented yet.");

        //Ok(())
    }

    // Takes &self since plotting performance is a read-only operation
    #[allow(dead_code)]
    fn plot_performance(&self) {
        // Add function name for easier debugging
        todo!("Function 'plot_performance' still needs to be implemented!");
    }
}