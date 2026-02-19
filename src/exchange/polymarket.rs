//! Polymarket CLOB client stub.
//!
//! Full implementation would use EIP-712 signing and REST API.
//! Enable the `polymarket` feature and add dependencies (e.g. ethers, reqwest) to implement.

use super::{Exchange, ExchangeError, Order, Side, Ticker};
use chrono::Utc;

/// Polymarket CLOB client (stub).
pub struct PolymarketClient {
    pub base_url: String,
    pub market_id: String,
}

impl PolymarketClient {
    pub fn new(_api_key: &str, _secret: &str, _passphrase: &str, _private_key_hex: &str, _funder_addr: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            base_url: "https://clob.polymarket.com".to_string(),
            market_id: String::new(),
        })
    }
}

impl Exchange for PolymarketClient {
    fn get_ticker(&self, market_id: &str) -> Result<Ticker, ExchangeError> {
        let _ = market_id;
        Err(ExchangeError::Fetch(anyhow::anyhow!(
            "Polymarket live GetTicker not implemented; use MockExchange for simulation"
        )))
    }

    fn place_order(
        &self,
        market_id: &str,
        side: Side,
        size: f64,
        price: f64,
    ) -> Result<Order, ExchangeError> {
        let _ = (market_id, side, size, price);
        Err(ExchangeError::Fetch(anyhow::anyhow!(
            "Polymarket PlaceOrder not implemented; use MockExchange for simulation"
        )))
    }

    fn current_time(&self) -> chrono::DateTime<Utc> {
        Utc::now()
    }
}
