//! Example demonstrating how to use the Bullish public candles endpoint
//!
//! This example shows how to:
//! - Get candlestick data for a market symbol
//! - Use different parameters to filter candles
//! - Display the candlestick information

use reqwest::Client;
use venues::bullish::public::RestClient;
use venues::bullish::{CandleInterval, CandleParams, RateLimiter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize HTTP client
    let client = Client::new();

    // Create rate limiter
    let rate_limiter = RateLimiter::new();

    // Create public REST client
    let public_client = RestClient::new(
        "https://api.exchange.bullish.com",
        client,
        rate_limiter,
    );

    // Get candles for BTCUSD with default parameters
    println!("Fetching BTCUSD candles (default parameters)...");
    match public_client.get_candles("BTCUSD", None).await {
        Ok(candles) => {
            println!("✅ BTCUSD Candles ({} candles):", candles.len());
            for (i, candle) in candles.iter().take(3).enumerate() {
                println!(
                    "  {}. Open: ${}, High: ${}, Low: ${}, Close: ${}, Volume: {} ({})",
                    i + 1,
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close,
                    candle.volume,
                    candle.open_time_datetime
                );
            }
            if candles.len() > 3 {
                println!("  ... and {} more", candles.len() - 3);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get BTCUSD candles: {}", e);
        }
    }

    println!();

    // Get candles with specific parameters
    println!("Fetching BTCUSD 1-hour candles (last 10)...");
    let params = CandleParams {
        interval: Some(CandleInterval::OneHour),
        limit: Some(10),
        ..Default::default()
    };

    match public_client.get_candles("BTCUSD", Some(params)).await {
        Ok(candles) => {
            println!("✅ BTCUSD 1-Hour Candles ({} candles):", candles.len());
            for (i, candle) in candles.iter().enumerate() {
                println!(
                    "  {}. {} - ${} -> ${} (Vol: {})",
                    i + 1,
                    candle.open_time_datetime,
                    candle.open,
                    candle.close,
                    candle.volume
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get BTCUSD 1-hour candles: {}", e);
        }
    }

    println!();

    // Get daily candles
    println!("Fetching BTCUSD daily candles (last 5)...");
    let daily_params = CandleParams {
        interval: Some(CandleInterval::OneDay),
        limit: Some(5),
        ..Default::default()
    };

    match public_client.get_candles("BTCUSD", Some(daily_params)).await {
        Ok(candles) => {
            println!("✅ BTCUSD Daily Candles ({} candles):", candles.len());
            for (i, candle) in candles.iter().enumerate() {
                let price_change = candle.close.parse::<f64>().unwrap_or(0.0)
                    - candle.open.parse::<f64>().unwrap_or(0.0);
                let change_direction = if price_change > 0.0 { "📈" } else { "📉" };
                
                println!(
                    "  {}. {} {} Open: ${} Close: ${} (Change: ${:.2})",
                    i + 1,
                    candle.open_time_datetime.split('T').next().unwrap_or(""),
                    change_direction,
                    candle.open,
                    candle.close,
                    price_change
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get BTCUSD daily candles: {}", e);
        }
    }

    Ok(())
}
