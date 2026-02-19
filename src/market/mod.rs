//! Price history buffer for dump detection.

use chrono::{DateTime, Duration, Utc};
use std::sync::RwLock;

/// A price at a specific time.
#[derive(Debug, Clone)]
pub struct PricePoint {
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}

/// Maintains a history of prices for dump detection.
pub struct PriceBuffer {
    history: RwLock<Vec<PricePoint>>,
    window: Duration,
}

impl PriceBuffer {
    pub fn new(window: Duration) -> Self {
        Self {
            history: RwLock::new(Vec::new()),
            window,
        }
    }

    /// Appends a new price point and removes points older than the window.
    pub fn add(&self, price: f64, ts: DateTime<Utc>) {
        let mut h = self.history.write().unwrap();
        h.push(PricePoint { price, timestamp: ts });

        let cutoff = ts - self.window;
        if let Some(valid_idx) = h.iter().position(|p| p.timestamp > cutoff) {
            if valid_idx > 0 {
                h.drain(..valid_idx);
            }
        }
    }

    /// Returns the price closest to `duration` ago from `now`, or None if insufficient history.
    pub fn get_price_ago(&self, duration: Duration, now: DateTime<Utc>) -> Option<f64> {
        let h = self.history.read().unwrap();
        if h.is_empty() {
            return None;
        }

        let target_time = now - duration;
        let mut best_price = -1.0_f64;
        let mut min_diff = chrono::Duration::hours(1);

        for p in h.iter() {
            let diff = (p.timestamp - target_time).abs();
            if diff < min_diff {
                min_diff = diff;
                best_price = p.price;
            }
        }

        if best_price < 0.0 {
            return None;
        }
        // If the best match is too far off (e.g. gap in data), return None.
        if min_diff > chrono::Duration::seconds(1) {
            return None;
        }
        Some(best_price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_price_buffer() {
        let pb = PriceBuffer::new(Duration::seconds(5));
        let now = Utc::now();

        pb.add(100.0, now - Duration::seconds(5));
        pb.add(102.0, now - Duration::seconds(3));
        pb.add(104.0, now - Duration::seconds(1));

        let price = pb.get_price_ago(Duration::seconds(3), now);
        assert_eq!(price, Some(102.0));

        let price = pb.get_price_ago(Duration::seconds(5), now);
        assert_eq!(price, Some(100.0));

        let price = pb.get_price_ago(Duration::seconds(1), now);
        assert_eq!(price, Some(104.0));
    }
}

