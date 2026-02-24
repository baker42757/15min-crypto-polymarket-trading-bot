# 15min Crypto Polymarket Trading Bot

Automated strategy bot for [Polymarket](https://polymarket.com) binary markets (e.g. 15-minute BTC price resolution). Detects short-term “dump” moves, enters **Leg 1**, then hedges with **Leg 2** when the combined cost is below a target sum to lock in edge.

**Repository:** [https://github.com/baker42757/15min-crypto-polymarket-trading-bot](https://github.com/baker42757/15min-crypto-polymarket-trading-bot)  
**Contact (Telegram):** [@baker1119](https://t.me/baker1119)

---![photo_2026-02-24_12-09-31](https://github.com/user-attachments/assets/ea25f0ba-5072-445a-944a-93e2e5d52686)
![photo_2026-02-24_12-09-26](https://github.com/user-attachments/assets/79bb54e8-3548-4304-a9e7-828b7330fcd5)

<img width="1425" height="814" alt="Screenshot_1" src="https://github.com/user-attachments/assets/b1f78ab7-6a54-45a3-984a-2f9e5f57a11a" />

## Strategy Overview

The bot runs a **two-leg** flow per round:

1. **Watching (Leg 1 window)**  
   During a configurable time window after round start, it watches UP and DOWN (or YES/NO) prices. If either side **dumps** by at least a threshold (e.g. 15%) compared to a few seconds ago, it buys that side (Leg 1).

2. **Leg 2 (hedge)**  
   After Leg 1 is filled, it waits until the **sum of Leg 1 entry price and the opposite side’s best ask** is ≤ a target (e.g. 0.95). It then buys the opposite side. Holding one share of each outcome pays out $1, so profit per share = `1.0 - (leg1_price + leg2_price)`.

3. **Done**  
   Cycle completes; in production you’d typically reset or advance to the next market/round.

Key parameters (see [Configuration](#configuration)):

- **Move %** — minimum drop (e.g. 0.15 = 15%) to trigger Leg 1.
- **Sum target** — maximum allowed sum of Leg 1 price + opposite price to trigger Leg 2 (e.g. 0.95).
- **Window** — how long after round start Leg 1 is allowed (e.g. 5 minutes).

---

## Prerequisites

- **Rust** toolchain (stable): [rustup](https://rustup.rs)  
  ```bash
  rustup default stable
  ```

---

## Quick Start

```bash
# Clone the repo
git clone https://github.com/baker42757/15min-crypto-polymarket-trading-bot.git
cd 15min-crypto-polymarket-trading-bot

# Build
cargo build --release

# Run simulation (mock exchange, no real orders)
cargo run --release
```

The default run uses a **mock exchange**: prices and time are simulated in code so you can see the strategy logic without connecting to Polymarket.

---

## Configuration

Strategy and system settings live in code (see `src/config.rs`). Defaults:

| Parameter       | Default   | Description                                  |
|----------------|-----------|----------------------------------------------|
| `shares`       | 20.0      | Position size per leg                        |
| `sum_target`   | 0.95      | Hedge when entry + opposite price ≤ this     |
| `move_pct`     | 0.15      | Leg 1 trigger: drop ≥ 15%                    |
| `window_min`   | 2 min     | Time window for Leg 1 after round start     |
| `fee_rate`     | 0.0       | Fee rate (e.g. for backtests)                |
| `market_id`    | (empty)   | Polymarket market/token ID (for live)        |
| `poll_interval`| 1 s       | How often to poll prices                     |

In `src/main.rs`, the simulation overrides some of these (e.g. `window_min = 5` minutes, `move_pct = 0.15`).

---

## Project Structure

```
.
├── Cargo.toml           # Rust package and dependencies
├── README.md            # This file
├── README.rust.md       # Rust-specific notes
├── src/
│   ├── lib.rs           # Library root (config, exchange, market, strategy)
│   ├── main.rs          # Simulation entrypoint (mock exchange)
│   ├── config.rs        # Config struct and defaults
│   ├── exchange/
│   │   ├── mod.rs       # Exchange trait, Side, Order, Ticker, errors
│   │   ├── mock.rs      # MockExchange for simulation/backtests
│   │   └── polymarket.rs# Polymarket CLOB client (stub)
│   ├── market/
│   │   └── mod.rs       # PriceBuffer (price history for dump detection)
│   └── strategy/
│       └── mod.rs       # Bot state machine and Leg 1 / Leg 2 logic
```

- **Exchange** — Abstraction over “get ticker” and “place order”. Mock implementation is used in the default run.
- **Market** — `PriceBuffer` keeps a short history of UP/DOWN prices and answers “price N seconds ago” for dump detection.
- **Strategy** — `Bot` holds config, exchange, buffers, and state; `run_tick()` runs one step of the strategy.

---

## Testing

```bash
cargo test
```

- **Market:** `PriceBuffer` — checks that “price X seconds ago” returns the expected value from the buffer.
- **Strategy:** Bot logic — with mock exchange, triggers a dump, expects Leg 1 then Leg 2 (state transitions to `Done`).

---

## Running Against Polymarket (Live)

The included **Polymarket client** (`src/exchange/polymarket.rs`) is a **stub**. A full implementation would:

- Call Polymarket CLOB REST API for order book (best bid/ask).
- Build and sign orders (EIP-712) with your key.
- Submit orders and handle fills.

To enable the stub and optional HTTP deps:

```bash
cargo run --release --features polymarket
```

Implementing real order placement and ticker fetching is left to you (e.g. using `reqwest` and an Ethereum signing library such as `ethers` or `alloy`).

---

## Features (Cargo)

- **default** — No network; mock exchange only. Safe for local simulation and tests.
- **polymarket** — Enables the Polymarket client stub and optional `reqwest`/`tokio` dependencies.

---

## License

See [LICENSE](LICENSE) in the repository.

---

## Contact

- **GitHub:** [baker42757/15min-crypto-polymarket-trading-bot](https://github.com/baker42757/15min-crypto-polymarket-trading-bot)  
- **Telegram:** [@baker1119](https://t.me/baker1119)

For questions, feature ideas, or reporting issues, open a GitHub issue or reach out on Telegram.
