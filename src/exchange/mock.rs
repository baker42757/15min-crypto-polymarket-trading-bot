//! Mock exchange for testing and backtesting.

use super::{Exchange, ExchangeError, Order, Side, Ticker};
use chrono::{DateTime, Duration, Utc};
use std::sync::{Arc, Mutex};

/// Simulates a market for testing/backtesting.
pub struct MockExchange {
    inner: Mutex<MockExchangeInner>,
}

struct MockExchangeInner {
    current_ticker: Ticker,
    time: DateTime<Utc>,
}

impl MockExchange {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(MockExchangeInner {
                time: Utc::now(),
                current_ticker: Ticker {
                    market_id: "mock-market".to_string(),
                    price_up: 0.50,
                    price_down: 0.50,
                    timestamp: Utc::now(),
                },
            }),
        }
    }

    pub fn set_price(&self, up: f64, down: f64) {
        let mut g = self.inner.lock().unwrap();
        g.current_ticker.price_up = up;
        g.current_ticker.price_down = down;
    }

    pub fn advance_time(&self, d: Duration) {
        let mut g = self.inner.lock().unwrap();
        g.time = g.time + d;
    }

    pub fn current_ticker(&self) -> Ticker {
        let g = self.inner.lock().unwrap();
        g.current_ticker.clone()
    }

    /// Simulates price movement (e.g. dump).
    pub fn simulate_dump(&self, side: Side, _from: f64, to: f64) {
        let mut g = self.inner.lock().unwrap();
        match side {
            Side::Up => {
                g.current_ticker.price_up = to;
            }
            Side::Down => {
                g.current_ticker.price_down = to;
            }
        }
    }
}

impl Default for MockExchange {
    fn default() -> Self {
        Self::new()
    }
}

impl Exchange for MockExchange {
    fn get_ticker(&self, _market_id: &str) -> Result<Ticker, ExchangeError> {
        let mut g = self.inner.lock().unwrap();
        g.current_ticker.timestamp = g.time;
        Ok(g.current_ticker.clone())
    }

    fn place_order(
        &self,
        market_id: &str,
        side: Side,
        size: f64,
        price: f64,
    ) -> Result<Order, ExchangeError> {
        if size <= 0.0 {
            return Err(ExchangeError::InvalidSize);
        }
        let time = self.inner.lock().unwrap().time;
        Ok(Order {
            id: "mock-order-id".to_string(),
            market_id: market_id.to_string(),
            side,
            price,
            size,
            timestamp: time,
        })
    }

    fn current_time(&self) -> DateTime<Utc> {
        self.inner.lock().unwrap().time
    }
}

impl Exchange for Arc<MockExchange> {
    fn get_ticker(&self, market_id: &str) -> Result<Ticker, ExchangeError> {
        self.as_ref().get_ticker(market_id)
    }

    fn place_order(
        &self,
        market_id: &str,
        side: Side,
        size: f64,
        price: f64,
    ) -> Result<Order, ExchangeError> {
        self.as_ref().place_order(market_id, side, size, price)
    }

    fn current_time(&self) -> DateTime<Utc> {
        self.as_ref().current_time()
    }
}
