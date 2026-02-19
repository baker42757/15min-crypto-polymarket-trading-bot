//! Exchange abstraction: types, trait, and implementations.

mod mock;
#[cfg(feature = "polymarket")]
mod polymarket;

pub use mock::MockExchange;
#[cfg(feature = "polymarket")]
pub use polymarket::PolymarketClient;

use chrono::{DateTime, Utc};
use thiserror::Error;

/// Outcome side (UP/DOWN or YES/NO).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Up,
    Down,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Up => write!(f, "UP"),
            Side::Down => write!(f, "DOWN"),
        }
    }
}

/// A trade order.
#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub market_id: String,
    pub side: Side,
    pub price: f64,
    pub size: f64,
    pub timestamp: DateTime<Utc>,
}

/// Current best prices.
#[derive(Debug, Clone)]
pub struct Ticker {
    pub market_id: String,
    pub price_up: f64,
    pub price_down: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum ExchangeError {
    #[error("invalid size")]
    InvalidSize,
    #[error("fetch error: {0}")]
    Fetch(#[from] anyhow::Error),
}

/// Interface for interacting with the market.
pub trait Exchange: Send + Sync {
    /// Returns the latest prices.
    fn get_ticker(&self, market_id: &str) -> Result<Ticker, ExchangeError>;

    /// Places a limit order (or market buy via limit).
    fn place_order(
        &self,
        market_id: &str,
        side: Side,
        size: f64,
        price: f64,
    ) -> Result<Order, ExchangeError>;

    /// Returns the exchange time (for backtesting).
    fn current_time(&self) -> DateTime<Utc>;
}
