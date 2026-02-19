//! Smart Ape strategy bot: watch for dump, leg 1, then hedge (leg 2).

use chrono::{Duration, Utc};

use crate::config::Config;
use crate::exchange::{Exchange, ExchangeError, Side, Ticker};
use crate::market::PriceBuffer;

/// Bot state in the cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Watching,
    Leg1Bought,
    Done,
}

pub struct Bot<E: Exchange> {
    cfg: Config,
    exchange: E,
    state: State,

    buffer_up: PriceBuffer,
    buffer_down: PriceBuffer,

    leg1_side: Option<Side>,
    leg1_entry_price: f64,
    round_start_time: chrono::DateTime<Utc>,
}

impl<E: Exchange> Bot<E> {
    pub fn new(cfg: Config, exchange: E) -> Self {
        let round_start_time = exchange.current_time();
        Self {
            cfg: cfg.clone(),
            exchange,
            state: State::Watching,
            buffer_up: PriceBuffer::new(Duration::seconds(5)),
            buffer_down: PriceBuffer::new(Duration::seconds(5)),
            leg1_side: None,
            leg1_entry_price: 0.0,
            round_start_time,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn leg1_side(&self) -> Option<Side> {
        self.leg1_side
    }

    /// Resets the bot for a new round.
    pub fn reset_cycle(&mut self) {
        eprintln!("--- Resetting Cycle for New Round ---");
        self.state = State::Watching;
        self.leg1_side = None;
        self.leg1_entry_price = 0.0;
        self.round_start_time = self.exchange.current_time();
    }

    /// Runs one tick of strategy logic.
    pub fn run_tick(&mut self) {
        let now = self.exchange.current_time();
        let ticker = match self.exchange.get_ticker(&self.cfg.market_id) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error fetching ticker: {}", e);
                return;
            }
        };

        self.buffer_up.add(ticker.price_up, now);
        self.buffer_down.add(ticker.price_down, now);

        match self.state {
            State::Watching => self.check_leg1(&ticker, now),
            State::Leg1Bought => self.check_leg2(&ticker),
            State::Done => {}
        }
    }

    fn check_leg1(&mut self, ticker: &Ticker, now: chrono::DateTime<Utc>) {
        let elapsed = now - self.round_start_time;
        if elapsed > self.cfg.window_min {
            return;
        }

        // Check UP dump
        if let Some(price_up_3s_ago) = self.buffer_up.get_price_ago(Duration::seconds(3), now) {
            let drop_pct = (price_up_3s_ago - ticker.price_up) / price_up_3s_ago;
            if drop_pct >= self.cfg.move_pct {
                eprintln!(
                    "DETECTED DUMP on UP! Drop: {:.2}% ({:.3} -> {:.3})",
                    drop_pct * 100.0,
                    price_up_3s_ago,
                    ticker.price_up
                );
                self.execute_leg1(Side::Up, ticker.price_up);
                return;
            }
        }

        // Check DOWN dump
        if let Some(price_down_3s_ago) = self.buffer_down.get_price_ago(Duration::seconds(3), now) {
            let drop_pct = (price_down_3s_ago - ticker.price_down) / price_down_3s_ago;
            if drop_pct >= self.cfg.move_pct {
                eprintln!(
                    "DETECTED DUMP on DOWN! Drop: {:.2}% ({:.3} -> {:.3})",
                    drop_pct * 100.0,
                    price_down_3s_ago,
                    ticker.price_down
                );
                self.execute_leg1(Side::Down, ticker.price_down);
            }
        }
    }

    fn execute_leg1(&mut self, side: Side, price: f64) {
        eprintln!(">>> EXECUTING LEG 1: Buy {} @ {:.3}", side, price);

        let order = match self.exchange.place_order(
            &self.cfg.market_id,
            side,
            self.cfg.shares,
            price,
        ) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Failed to place Leg 1 order: {}", e);
                return;
            }
        };

        self.leg1_side = Some(side);
        self.leg1_entry_price = order.price;
        self.state = State::Leg1Bought;
        eprintln!(
            "Leg 1 Filled. Waiting for Hedge (Target Sum <= {:.2})...",
            self.cfg.sum_target
        );
    }

    fn check_leg2(&mut self, ticker: &Ticker) {
        let side = match self.leg1_side {
            Some(s) => s,
            None => return,
        };

        let (opposite_price, opposite_side) = match side {
            Side::Up => (ticker.price_down, Side::Down),
            Side::Down => (ticker.price_up, Side::Up),
        };

        let current_sum = self.leg1_entry_price + opposite_price;

        if current_sum <= self.cfg.sum_target {
            eprintln!(
                "HEDGE CONDITION MET! Sum: {:.3} (Entry: {:.3} + Opp: {:.3}) <= Target: {:.3}",
                current_sum,
                self.leg1_entry_price,
                opposite_price,
                self.cfg.sum_target
            );
            self.execute_leg2(opposite_side, opposite_price);
        }
    }

    fn execute_leg2(&mut self, side: Side, price: f64) {
        eprintln!(">>> EXECUTING LEG 2 (HEDGE): Buy {} @ {:.3}", side, price);

        let order = match self.exchange.place_order(
            &self.cfg.market_id,
            side,
            self.cfg.shares,
            price,
        ) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Failed to place Leg 2 order: {}", e);
                return;
            }
        };

        let total_cost = self.leg1_entry_price + order.price;
        let profit = 1.0 - total_cost;
        let roi = (profit / total_cost) * 100.0;

        eprintln!(
            "CYCLE COMPLETE. Total Cost: {:.3}, Profit per share: {:.3}, ROI: {:.2}%",
            total_cost, profit, roi
        );
        self.state = State::Done;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exchange::{MockExchange, Side};
    use chrono::Duration;
    use std::sync::Arc;

    #[test]
    fn test_bot_logic() {
        let mut cfg = Config::default();
        cfg.move_pct = 0.10;
        cfg.sum_target = 0.96;

        let mock_exc = Arc::new(MockExchange::new());
        let mut bot = Bot::new(cfg, Arc::clone(&mock_exc));

        mock_exc.set_price(0.50, 0.50);
        bot.run_tick();

        mock_exc.advance_time(Duration::seconds(3));
        bot.run_tick();

        mock_exc.advance_time(Duration::seconds(1));
        mock_exc.set_price(0.40, 0.55);
        bot.run_tick();

        assert_eq!(bot.state(), State::Leg1Bought);
        assert_eq!(bot.leg1_side(), Some(Side::Up));

        bot.run_tick();

        assert_eq!(bot.state(), State::Done);
    }
}
