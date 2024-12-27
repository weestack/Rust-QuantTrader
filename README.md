
# Rust Backtester

A **Rust-based trading library** designed for professionals and hobbyists alike. This package offers powerful, modular tools for creating a robust trading ecosystem, including backtesting, custom strategies, data fetching, exchange implementations, and even your own **technical analysis (TA) library**.

---

## Features (Roadmap for Alpha, we are not in alpha yet, so no expectations)

- ⚡ **Backtesting Framework**: Test your trading strategies with historical data to evaluate performance before going live.
- 🏗️ **Custom Strategies**: Implement and plug in your own trading strategies.
- 📊 **Data Fetching**: Fetch and process market data for all your backtesting and live trading needs.
- 🔄 **Exchange Implementations**: Modules for interfacing with real or simulated trading exchanges.
- 📈 **Custom TA Library**: Develop your own indicators, moving averages, or other technical analysis tools.

---

## Modules Overview

1. **Backtesting**:
    - Provides a seamless interface to test strategies against historical data.
    - Example usage:
      ```rust
      /* TODO write mockup implementation */
      println!("{:?}", results.performance());
      ```

2. **Custom Strategies**:
    - Create strategies tailored to your needs by implementing simple traits or callback logic:
      ```rust
      /* TODO write strategy example */
      struct MyStrategy;
      
      impl Strategy for MyStrategy {
          fn execute(&self, data: &Data) -> Signal {
              // Custom logic here
          }
      }
      ```

3. **Data Fetching**:
    - Fetch data from exchanges or use offline files for testing purposes.

4. **Exchange Implementations**:
    - Easily integrate your code with real-world trading exchanges or simulate trades to test strategies.

5. **Custom TA Library**:
    - Build your own indicators such as moving averages, momentum oscillators, etc.

---

## Example Usage

```rust
/* TODO write example usage */
use backtester::backtesting::Backtester;
use backtester::strategies::SimpleMomentumStrategy;
use backtester::data::fetch::fetch_data;

fn main() {
    // Fetch historical data
    let data = fetch_data("BTC/USD", "binance").unwrap();
    
    // Define your strategy
    let strategy = SimpleMomentumStrategy::new();

    // Perform backtest
    let backtest_results = Backtester::new(strategy, data).run();

    // Display results
    println!("{:?}", backtest_results.summary());
}
```

---

## Contributing

Contributions are always welcome! If you have suggestions for new features, bug fixes, or simply wish to contribute code, feel free to submit a pull request.

---

## Supporting My Work

Your support will help me continue to develop these tools and create more useful resources for the crypto or trading community.

- **Solana (SOL)**: `[EyE5XKNGB2o7JrpgbfdiF5RU9dWa5VQ4gekjgFQ1kFDv]`
- **Ethereum (ETH)**: `[0x31bf3487139bed31a646ac5863e2fa115f5d9fee]`

---

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use it as you wish. Attribution is appreciated but not required.

---

## Roadmap and TODO's

- [ ] Refactor the **backtesting module** for better performance and modularity.
- [ ] Implement candle streaming
- 
- [ ] Add more exchange integrations (e.g., Kraken, Binance).

---
## Usefull commands section
```bash
cargo test test_strategy_with_backtrader --package Backtester -- --show-output 
```# Rust-QuantTrader
