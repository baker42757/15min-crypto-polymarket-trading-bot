//! Polymarket Smart Ape strategy bot — simulation / mock mode.

use chrono::Duration;
use std::sync::Arc;

use poly::config::Config;
use poly::exchange::MockExchange;
use poly::strategy::Bot;

fn main() {
    eprintln!("启动 Polymarket Smart Ape 策略机器人 (模拟模式)...");

    let mut cfg = Config::default();
    cfg.window_min = Duration::minutes(5);
    cfg.move_pct = 0.15; // 15% drop

    let mock_exc = Arc::new(MockExchange::new());
    let mut bot = Bot::new(cfg.clone(), Arc::clone(&mock_exc));

    eprintln!(">>> 模拟开始: 初始价格 UP: 0.50, DOWN: 0.50");
    mock_exc.set_price(0.50, 0.50);

    for _ in 0..10 {
        mock_exc.advance_time(Duration::seconds(1));
        bot.run_tick();
    }

    eprintln!("\n>>> 模拟暴跌事件! UP 0.50 -> 0.30");
    mock_exc.advance_time(Duration::seconds(1));
    mock_exc.set_price(0.30, 0.55);
    bot.run_tick();

    eprintln!("\n>>> 模拟暴跌场景 2: DOWN 价格飙升导致无法立即对冲");
    bot.reset_cycle();
    mock_exc.set_price(0.50, 0.50);
    for _ in 0..5 {
        mock_exc.advance_time(Duration::seconds(1));
        bot.run_tick();
    }

    eprintln!(">>> 暴跌发生...");
    mock_exc.advance_time(Duration::seconds(1));
    mock_exc.set_price(0.30, 0.75);
    bot.run_tick();

    eprintln!("\n>>> 等待对冲机会...");
    for step in 1..=11 {
        mock_exc.advance_time(Duration::seconds(1));
        let t = mock_exc.current_ticker();
        let mut down = t.price_down;
        if down > 0.60 {
            down -= 0.02;
            mock_exc.set_price(0.30, down);
        }
        eprintln!(
            "Tick {}: UP={:.2}, DOWN={:.2}",
            step,
            mock_exc.current_ticker().price_up,
            mock_exc.current_ticker().price_down
        );
        bot.run_tick();
    }
}
