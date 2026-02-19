# Polymarket Smart Ape Bot — Rust Port

This directory contains the Rust implementation of the 15min Polymarket trading bot.

**Main README:** [README.md](README.md) — strategy overview, setup, configuration, contact.  
**Repo:** [github.com/0xbulli/15min-crypto-polymarket-trading-bot](https://github.com/0xbulli/15min-crypto-polymarket-trading-bot) · **Telegram:** [@gabagool222](https://t.me/gabagool222)

## Structure

| Go | Rust |
|----|------|
| `main.go` | `src/main.rs` |
| `pkg/config` | `src/config.rs` |
| `pkg/exchange` (interface, mock, polymarket) | `src/exchange/mod.rs`, `mock.rs`, `polymarket.rs` |
| `pkg/market` (buffer) | `src/market/mod.rs` |
| `pkg/strategy` (bot) | `src/strategy/mod.rs` |

## Build & Run

```bash
cargo build
cargo run
```

## Test

```bash
cargo test
```

## Features

- **default**: Mock exchange only (no network).
- **polymarket**: Enables the Polymarket client stub (`PolymarketClient`). A full CLOB implementation would require EIP-712 signing (e.g. with `ethers` or `alloy`) and REST calls.

## Dependencies

- `chrono` — time and duration
- `anyhow` / `thiserror` — errors
- `serde` / `serde_json` — config serialization (optional)

The Go `PolymarketClient` (EIP-712 signing, orderbook API) is only stubbed in Rust; you can implement it using `reqwest` and an Ethereum signing library when needed.
