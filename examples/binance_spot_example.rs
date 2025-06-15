// Example usage of the Binance Spot Public REST Client
use venues::binance::spot::{PublicRestClient, RateLimiter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client and rate limiter
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();
    
    // Create the Binance Spot public REST client
    let binance_spot = PublicRestClient::new(
        "https://api.binance.com", 
        client, 
        rate_limiter
    );
    
    // Test connectivity
    println!("Testing ping...");
    match binance_spot.ping().await {
        Ok(response) => println!("Ping successful: {:?}", response.data),
        Err(e) => println!("Ping failed: {:?}", e),
    }
    
    // Get server time
    println!("Getting server time...");
    match binance_spot.time().await {
        Ok(response) => println!("Server time: {:?}", response.data),
        Err(e) => println!("Time request failed: {:?}", e),
    }
    
    // Get exchange info
    println!("Getting exchange info...");
    match binance_spot.exchange_info().await {
        Ok(response) => {
            println!("Exchange timezone: {}", response.data.timezone);
            println!("Server time: {}", response.data.server_time);
            println!("Number of symbols: {}", response.data.symbols.len());
        },
        Err(e) => println!("Exchange info failed: {:?}", e),
    }
    
    // Get order book for BTCUSDT
    println!("Getting order book for BTCUSDT...");
    match binance_spot.depth("BTCUSDT", Some(5)).await {
        Ok(response) => println!("Order book: {:?}", response.data),
        Err(e) => println!("Order book failed: {:?}", e),
    }
    
    // Get recent trades for BTCUSDT
    println!("Getting recent trades for BTCUSDT...");
    match binance_spot.trades("BTCUSDT", Some(5)).await {
        Ok(response) => println!("Recent trades: {:?}", response.data),
        Err(e) => println!("Trades failed: {:?}", e),
    }
    
    // Get 24hr ticker for BTCUSDT
    println!("Getting 24hr ticker for BTCUSDT...");
    match binance_spot.ticker_24hr(Some("BTCUSDT"), None, None).await {
        Ok(response) => println!("24hr ticker: {:?}", response.data),
        Err(e) => println!("24hr ticker failed: {:?}", e),
    }
    
    Ok(())
}