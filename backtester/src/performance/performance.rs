pub fn calculate_total_return(final_portfolio_value: f64, initial_capital: f64) -> f64 {
    (final_portfolio_value / initial_capital) - 1.0
}

pub fn calculate_annualized_return(total_return: f64, num_days: u32) -> f64 {
    //"""Calculate the annualized return of the portfolio."""
    (1.0 + total_return).powf(252.0 / num_days as f64) - 1.0
}

pub fn calculate_annualized_volatility(_daily_returns: Vec<f64>) -> Result<f64, &'static str> {
    // Calculate the standard deviation
    todo!("implement calculate_annualized_volatility");
}

pub fn calculate_sharpe_ratio(_annualized_return: f64, _annualized_volatility: f64, _risk_free_rate: f64) -> f64 {
    // Calculate the standard deviation
    todo!("implement calculate_sharpe_ratio");
    //(annualized_return - risk_free_rate) / annualized_volatility
}


pub fn calculate_sortino_ratio(_daily_returns: Vec<f64>, _annualized_return: f64, _risk_free_rate: f64) -> f64 {
    // Calculate the standard deviation
    /*
    negative_returns = daily_returns[daily_returns < 0]
    downside_volatility = negative_returns.std() * np.sqrt(252)
    return (
    (annualized_return - risk_free_rate) / downside_volatility
        if downside_volatility > 0
        else np.nan
    )

    */
    todo!("implement calculate_sortino_ratio");
    //annualized_return - risk_free_rate
}

pub fn calculate_maximum_drawdown(_portfolio_values: Vec<f64>) -> f64 {
    //drawdown = portfolio_values / portfolio_values.cummax() - 1
    //drawdown.min()
    todo!("Implement calculate_maximum_drawdown");
    //0.00
}