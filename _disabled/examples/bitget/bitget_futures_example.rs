/// Example of using Bitget API
///
/// This example demonstrates how to use the Bitget API endpoints for both public and private data.
use reqwest::Client;
use venues::bitget::{
    CandlestickGranularity, PricePrecision, PublicRestClient, RateLimiter,
    public::rest::{GetCandlestickRequest, GetMergeDepthRequest, GetTickerRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = PublicRestClient::new(
        "https://api.bitget.com".to_string(),
        RateLimiter::default(),
        Client::new(),
    );

    // Public Endpoints Examples
    println!("=== Bitget Public API Examples ===");

    // Get VIP fee rates
    println!("Getting VIP fee rates...");
    match client.get_vip_fee_rate().await {
        Ok(response) => {
            println!("VIP fee rates retrieved successfully!");
            for (i, rate) in response.data.iter().enumerate() {
                if i < 3 {
                    // Show first 3 rates
                    println!(
                        "Level {}: Taker {}%, Maker {}%",
                        rate.level, rate.taker_fee_rate, rate.maker_fee_rate
                    );
                }
            }
        }
        Err(e) => println!("Failed to get VIP fee rates: {:?}", e),
    }

    // Get ticker for a specific symbol
    let ticker_request = GetTickerRequest {
        symbol: Some("BTCUSDT".to_string()),
    };
    println!("\nGetting ticker for BTCUSDT...");
    match client.get_ticker(&ticker_request).await {
        Ok(response) => {
            if let Some(ticker) = response.data.first() {
                println!(
                    "BTCUSDT Price: {} (24h high: {}, low: {})",
                    ticker.last_price, ticker.high24h, ticker.low24h
                );
            }
        }
        Err(e) => println!("Failed to get ticker: {:?}", e),
    }

    // Get all tickers
    let all_tickers_request = GetTickerRequest { symbol: None };
    println!("\nGetting all tickers...");
    match client.get_ticker(&all_tickers_request).await {
        Ok(response) => {
            println!("Retrieved {} tickers", response.data.len());
        }
        Err(e) => println!("Failed to get all tickers: {:?}", e),
    }

    // Get market depth
    let depth_request = GetMergeDepthRequest {
        symbol: "BTCUSDT".to_string(),
        precision: Some(PricePrecision::Scale0),
        limit: Some(50),
    };
    println!("\nGetting market depth for BTCUSDT...");
    match client.get_merge_depth(&depth_request).await {
        Ok(response) => {
            println!(
                "Market depth retrieved with {} asks and {} bids",
                response.data.asks.len(),
                response.data.bids.len()
            );
        }
        Err(e) => println!("Failed to get market depth: {:?}", e),
    }

    // Get candlestick data
    let candle_request = GetCandlestickRequest {
        symbol: "BTCUSDT".to_string(),
        granularity: CandlestickGranularity::OneHour,
        start_time: None,
        end_time: None,
        limit: Some(10),
    };
    println!("\nGetting candlestick data for BTCUSDT...");
    match client.get_candlestick(&candle_request).await {
        Ok(response) => {
            println!("Retrieved {} candlesticks", response.data.len());
            if let Some(candle) = response.data.first() {
                // Candlestick is [String; 8] with format: [timestamp, open, high, low, close, volume, quote_volume, count]
                if candle.len() >= 5 {
                    println!(
                        "Latest candle: O:{} H:{} L:{} C:{}",
                        candle[1], candle[2], candle[3], candle[4]
                    );
                }
            }
        }
        Err(e) => println!("Failed to get candlestick data: {:?}", e),
    }

    println!("\nBitget API example completed successfully!");
    Ok(())
}
