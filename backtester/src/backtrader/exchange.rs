#[derive(Debug)]
pub struct Exchange {
    pub name: String,
    pub commission_pct: f64,
    pub commission_fixed: f64,
}

impl Exchange {
    // Takes &self since it doesn't modify the Backtrader instance
    pub fn calculate_commission(&mut self, trade_value: f64) -> f64 {
        // Add function name for easier debugging
        (trade_value * self.commission_pct).max(self.commission_fixed)
    }
}