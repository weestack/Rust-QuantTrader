use polars::prelude::*;
pub fn calculate_total_return(final_portfolio_value: f64, initial_capital: f64) -> f64 {
    (final_portfolio_value / initial_capital) - 1.0
}

fn calculate_annualized_base(total_return: f64, ticker: f64, num: u32) -> f64 {
    // (1 + Return) ^ (ticker / N) - 1 = Annualized Return
    // https://www.investopedia.com/terms/a/annualized-total-return.asp
    (1.0 + total_return).powf(ticker / num as f64) - 1.0
}

pub fn calculate_annualized_yearly_return(total_return: f64, num_years: u32) -> f64 {
    // (1 + Return) ^ (1 / N) - 1 = Annualized Return
    calculate_annualized_base(total_return, 1.0, num_years)
}

pub fn calculate_annualized_daily_return(total_return: f64, num_days: u32) -> f64 {
    // (1 + Return) ^ (365 / N) - 1 = Annualized Return // 365 trading days in crypto
    calculate_annualized_base(total_return, 365.0, num_days)
}


pub fn calculate_annualized_volatility(daily_returns: Series, ticker: f64) -> f64 {
    daily_returns.std(1).unwrap() * ticker.sqrt()
}

pub fn calculate_annualized_yearly_volatility(daily_returns: Series) -> f64 {
    calculate_annualized_volatility(daily_returns, 1.0)
}


pub fn calculate_annualized_daily_volatility(daily_returns: Series) -> f64 {
    calculate_annualized_volatility(daily_returns, 365.0)
}

pub fn calculate_sharpe_ratio(annualized_return: f64, annualized_volatility: f64, risk_free_rate: f64) -> f64 {
    (annualized_return - risk_free_rate) / annualized_volatility
}


pub fn calculate_sortino_ratio(daily_returns: Series, annualized_return: f64, risk_free_rate: f64) -> f64 {
    //let negative_returns = daily_returns.f64().expect("Not f64").get(0.0).into_series();
    0.0
}

pub fn calculate_maximum_drawdown(_portfolio_values: Vec<f64>) -> f64 {
    //drawdown = portfolio_values / portfolio_values.cummax() - 1
    //drawdown.min()
    todo!("Implement calculate_maximum_drawdown");
    //0.00
}

pub fn calculate_daily_returns(daily_values: &Vec<f64>) -> Result<DataFrame, PolarsError> {
    /* TODO Store Portfo    lio values and daily values as Dataframe in some extraction, to make it faster for final data crunching */

    // Convert the daily portfolio values to a Polars DataFrame/Series
    //let series = Series::new("portfolio_values".into(), daily_values.clone());
    let frame = df!(
        "daily_values" => daily_values.clone()
    )?;

    //let pcv_change = series.lit().pct_change(col("portfolio_values")).drop_nans();
        //DataFrame::new(PlSmallStr::from("portfolio_values"), daily_values.clone());
    // Calculate percentage change
    let pct_change = frame
        .lazy()
        .with_columns([col("daily_values").pct_change(lit(1)).alias("pct_change")])
        .select([col("pct_change").drop_nans()])
        .collect()?;

    Ok(pct_change)
}

mod tests {
    use super::*;
    #[test]
    fn test_calculate_total_return() {
        let total_return_0 = calculate_total_return(100.0, 100.0);
        assert_eq!(total_return_0, 0.0); // 0%

        let total_return_50 = calculate_total_return(150.0, 100.0);
        assert_eq!(total_return_50, 0.5); // 50%

        let total_return_crash = calculate_total_return(10.0, 0.51);
        assert_eq!(total_return_crash, 18.607843137254903); // 18.6%

        let total_return_98 = calculate_total_return(23.1, 100.0);
        assert_eq!(total_return_98, -0.769); // -76.9%
    }

    #[test]
    fn test_calculate_annualized_return() {
        // test case of 250% yield

        // daily performance 30 days held
        let annualized_return_days = calculate_annualized_daily_return(2.5, 5);
        assert_eq!(annualized_return_days, 5.2115539455415427e39);

        // yearly performance 5 years held
        let annualized_return_weeks = calculate_annualized_yearly_return(2.5, 5);
        assert_eq!(annualized_return_weeks, 0.28473515712343933);
    }


    #[test]
    fn test_calculate_annualized_volatility() {
        let daily_returns = Series::new("daily_returns".into(), vec![0.01, 0.02, 0.03, 0.04, 0.05, 0.1, -0.3, -0.9, 0.4]);

        // daily performance 30 days held
        let annualized_volatility_days = calculate_annualized_daily_volatility(daily_returns.clone());
        assert_eq!(annualized_volatility_days, 6.894648689785112);

        // yearly performance 5 years held
        let annualized_return_weeks = calculate_annualized_yearly_volatility(daily_returns);
        assert_eq!(annualized_return_weeks, 0.36088240620888007);
    }


    #[test]
    fn test_calculate_sharpe_ratio_mid() {
        let annualized_return = 0.01;
        let annualized_volatility = 0.02;
        let risk_free_rate = 0.0;
        let sharpe_ratio = calculate_sharpe_ratio(annualized_return, annualized_volatility, risk_free_rate);
        assert_eq!(sharpe_ratio, 0.5);
    }

    #[test]
    fn test_calculate_sharpe_ratio_high() {
        let annualized_return = 0.92;
        let annualized_volatility = 0.19;
        let risk_free_rate = 0.0;
        let sharpe_ratio = calculate_sharpe_ratio(annualized_return, annualized_volatility, risk_free_rate);
        assert_eq!(sharpe_ratio, 4.842105263157895);
    }

    #[test]
    fn test_calculate_sortino_ratio() {
        let daily_returns = Series::new("daily_returns".into(), vec![0.01, 0.02, 0.03, 0.04, 0.05, 0.1, -0.3, -0.9, 0.4]);
        let annualized_return = 0.01;
        let risk_free_rate = 0.0;
        let sortino_ratio = calculate_sortino_ratio(daily_returns, annualized_return, risk_free_rate);
        assert_eq!(sortino_ratio, 0.0);
    }
}