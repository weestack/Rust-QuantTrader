use polars::prelude::*;
use std::collections::HashMap;
use polars::export::num::CheckedSub;
use crate::backtrader::asset_data::AssetData;
use crate::backtrader::exchange::Exchange;
use crate::performance::performance::{calculate_annualized_yearly_return, calculate_annualized_volatility, calculate_daily_returns, calculate_total_return};
use crate::strategy::strategy::{StrategyTrait};

pub type PortfolioHistory = HashMap<String, Vec<f64>>;
pub type DailyPortfolioValues = HashMap<String, Vec<f64>>;

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
        let mut portfolio_history: HashMap<String, Vec<f64>> = HashMap::new();
        let mut daily_portfolio_values: HashMap<String, Vec<f64>> = HashMap::new();
        let symbol_capital = initial_capital / symbols.len() as f64;
        for symbol in symbols {
            let asset_data = AssetData::new(symbol.as_str(), symbol_capital, 0.0, 0.0);
            assets_data.insert(symbol.clone(), asset_data.clone());
            portfolio_history.insert(symbol.clone(), vec![]);
            daily_portfolio_values.insert(symbol.clone(), vec![]);
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
            portfolio_history,
            daily_portfolio_values: daily_portfolio_values.clone(),
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
                //asset.position_value = asset.positions * price;
                //asset.total_value = asset.cash + asset.position_value;

                // Update total value history for tracking
                //asset.history.push(asset.total_value);
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
            if let Some(last_value) = self.daily_portfolio_values.get_mut(symbol).unwrap().last_mut() {
                *last_value += asset.total_value;
            } else {
                self.daily_portfolio_values.get_mut(symbol).unwrap().push(asset.total_value);
            }
        } else {
            eprintln!("Asset '{}' not found in portfolio, cannot update portfolio.", symbol);
        }
    }

    // Takes &mut self since it likely modifies or interacts with the Backtrader instance during the backtest process
    pub fn backtest(&mut self, symbol: Option<String>, strategy: impl StrategyTrait) -> Result<(), PolarsError> {
        // Split initial capital equally among all assets in data
        let assets_count = self.assets_data.len() as f64;

        let symbols = if symbol.is_some() {
            vec![symbol.unwrap()]
        } else {
            self.assets_data.keys().cloned().collect()
        };

        for symbol in symbols.clone() {
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

                self.execute_trade(&symbol.clone(), signal, price);
                self.update_portfolio(&symbol.clone(), price);

                let portfolio = self.portfolio_history.get_mut(&symbol.clone()).unwrap();
                let new_value = self.assets_data.get(&symbol).unwrap().total_value;
                if portfolio.is_empty() || portfolio.last().unwrap() != &new_value {
                    /* TODO perhaps store with timestamp? */
                    portfolio.push(new_value);
                }

                let daily = self.daily_portfolio_values.get_mut(&symbol.clone()).unwrap().last_mut().unwrap();
                *daily += self.assets_data.get(&symbol).unwrap().total_value;

                /*let index = self.portfolio_history.get(&symbol.clone()).unwrap().len().checked_sub(1).unwrap();
                self.portfolio_history.get_mut(&symbol.clone()).unwrap().insert(
                    index,
                    self.assets_data.get(&symbol).unwrap().total_value
                );
                let last = self.daily_portfolio_values.get_mut(&symbol.clone()).unwrap().last_mut().unwrap();
                *last += self.assets_data.get_mut(&symbol).unwrap().total_value;*/
            }
        }

        Ok(())
    }


    // Takes &self since performance calculation likely doesn't modify the Backtrader instance
    pub fn calculate_performance(&self, _plot: bool /* TODO implement plotting */ ) -> Result<(), PolarsError> {
        if self.daily_portfolio_values.is_empty() {
            println!("No daily portfolio values found nothing to calculate performance over.");
            return Ok(())
        }
        println!("Calculating performance over {:?}", self.portfolio_history);
        println!("Calculating daily performance over {:?}", self.daily_portfolio_values);
        // Add function name for easier debugging


        let mut total_portfolio_value = 0.0;
        /* TODO print both total and for each different symbol */
        for (_, value) in self.daily_portfolio_values.iter() {
            total_portfolio_value += value.last().unwrap();
        }

        println!("Final Portfolio Value: {:.2}", total_portfolio_value);

        if self.daily_portfolio_values.len() > 0 {
            for (symbol, value) in self.daily_portfolio_values.iter() {
                println!("Final value for pair {}: {:.2}", symbol, value.last().unwrap());
            }
        }

        let total_return = calculate_total_return(total_portfolio_value, self.initial_capital);
        println!("Total Return: {:.2}%", total_return * 100.0);

        /* TODO naive approach, use a ticker implementation to get the yearly yield */

        let annualized_return = calculate_annualized_yearly_return(total_return, self.daily_portfolio_values.len() as u32);
        println!("Annualized Return: {:.2}%", annualized_return * 100.0);

        let daily_returns = calculate_daily_returns(self.daily_portfolio_values.get("BTCUSDT").unwrap());
        println!("Daily Returns: {:?}", daily_returns);
        //let annualized_volatility = calculate_annualized_volatility(daily_returns);
        //println!("Annualized Volatility: {:.2}%", annualized_volatility * 100.0);
        //println!("Sharpe Ratio: {sharpe_ratio:.2f}");
        //println!("Sortino Ratio: {sortino_ratio:.2f}");
        //println!("Maximum Drawdown: {max_drawdown * 100:.2f}%");

        /* TODO remove when all metrics works! */
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