//! Strategy and system configuration.

use chrono::Duration;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// Position size (e.g. 20)
    #[serde(rename = "shares")]
    pub shares: f64,

    /// Hedge threshold (e.g. 0.95)
    #[serde(rename = "sum_target")]
    pub sum_target: f64,

    /// Dump threshold (e.g. 0.15 for 15%)
    #[serde(rename = "move_pct")]
    pub move_pct: f64,

    /// Time window for Leg 1 (e.g. 2 minutes)
    #[serde(rename = "window_min")]
    pub window_min: Duration,

    /// Fee rate for simulation (e.g. 0.001)
    #[serde(rename = "fee_rate")]
    pub fee_rate: f64,

    /// The Market ID to trade
    #[serde(rename = "market_id")]
    pub market_id: String,

    /// Poll interval
    #[serde(rename = "poll_interval")]
    pub poll_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shares: 20.0,
            sum_target: 0.95,
            move_pct: 0.15,
            window_min: Duration::minutes(2),
            fee_rate: 0.0,
            market_id: String::new(),
            poll_interval: Duration::seconds(1),
        }
    }
}
