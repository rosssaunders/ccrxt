//! Example demonstrating how to get real-time market data from Bullish Exchange
//!
//! This example shows how to:
//! 1. Get current market ticker information
//! 2. Fetch the order book
//! 3. Retrieve recent public trades
//! 4. Get server time for synchronization

use venues::bullish::public::RestClient;
use venues::bullish::{OrderbookParams, PublicTradesParams, RateLimiter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client and rate limiter
    let http_client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();

    // Create public REST client
    let client = RestClient::new(
        "https://api.exchange.bullish.com",
        http_client,
        rate_limiter,
    );

    println!("🚀 Getting real-time market data from Bullish Exchange...");

    // Get server time for synchronization
    match client.get_server_time().await {
        Ok(server_time) => {
            println!("🕐 Server time: {} ({})", server_time.datetime, server_time.timestamp);
        }
        Err(e) => println!("❌ Failed to get server time: {}", e),
    }

    // Get ticker information for BTCUSDC
    let symbol = "BTCUSDC";
    println!("\n📊 Getting ticker for {}...", symbol);

    match client.get_ticker(symbol).await {
        Ok(ticker) => {
            println!("Ticker Information:");
            println!("  Symbol: {}", ticker.symbol);
            println!("  Last Price: ${}", ticker.last_price);
            println!("  24h Change: {}%", ticker.price_change_percent);
            println!("  24h High: ${}", ticker.high_price);
            println!("  24h Low: ${}", ticker.low_price);
            println!("  24h Volume: {}", ticker.volume);
            println!("  24h Quote Volume: ${}", ticker.quote_volume);
            println!("  Best Bid: ${} ({})", ticker.bid_price, ticker.bid_qty);
            println!("  Best Ask: ${} ({})", ticker.ask_price, ticker.ask_qty);
            println!("  Trades Count: {}", ticker.count);
        }
        Err(e) => println!("❌ Failed to get ticker: {}", e),
    }

    // Get order book
    println!("\n📈 Getting order book for {}...", symbol);
    let orderbook_params = OrderbookParams {
        depth: Some(10), // Get top 10 levels
        aggregate: Some(true),
    };

    match client.get_orderbook(symbol, Some(orderbook_params)).await {
        Ok(orderbook) => {
            println!("Order Book (Top 5 levels):");
            println!("  Sequence: {}", orderbook.sequence);
            println!("  Timestamp: {}", orderbook.timestamp);
            
            println!("\n  Asks (Sell Orders):");
            for (i, ask) in orderbook.asks.iter().take(5).enumerate() {
                println!("    {}. ${} x {}", i + 1, ask.price, ask.quantity);
            }
            
            println!("\n  Bids (Buy Orders):");
            for (i, bid) in orderbook.bids.iter().take(5).enumerate() {
                println!("    {}. ${} x {}", i + 1, bid.price, bid.quantity);
            }

            // Calculate spread
            if let (Some(best_ask), Some(best_bid)) = (orderbook.asks.first(), orderbook.bids.first()) {
                let ask_price: f64 = best_ask.price.parse().unwrap_or(0.0);
                let bid_price: f64 = best_bid.price.parse().unwrap_or(0.0);
                let spread = ask_price - bid_price;
                let spread_percent = (spread / bid_price) * 100.0;
                println!("\n  Spread: ${:.2} ({:.4}%)", spread, spread_percent);
            }
        }
        Err(e) => println!("❌ Failed to get order book: {}", e),
    }

    // Get recent public trades
    println!("\n💱 Getting recent public trades for {}...", symbol);
    let trades_params = PublicTradesParams {
        limit: Some(10),
        ..Default::default()
    };

    match client.get_public_trades(symbol, Some(trades_params)).await {
        Ok(trades) => {
            println!("Recent Trades:");
            for (i, trade) in trades.iter().take(5).enumerate() {
                println!(
                    "  {}. {} {} {} @ ${} ({})",
                    i + 1,
                    trade.side as u8,
                    trade.quantity,
                    trade.symbol,
                    trade.price,
                    trade.datetime
                );
            }

            if !trades.is_empty() {
                let total_volume: f64 = trades
                    .iter()
                    .map(|t| t.quantity.parse::<f64>().unwrap_or(0.0))
                    .sum();
                println!("  Total Volume (last {} trades): {:.4}", trades.len(), total_volume);
            }
        }
        Err(e) => println!("❌ Failed to get public trades: {}", e),
    }

    println!("\n✨ Market data example completed!");
    Ok(())
}
