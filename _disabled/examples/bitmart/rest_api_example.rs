//! BitMart REST API example
//!
//! This example demonstrates how to use the BitMart REST API for both public and private endpoints.

use venues::bitmart::{
    PrivateRestClient, PublicRestClient, GetTickerRequest, SubmitOrderRequest,
    OrderSide, OrderType, StpMode,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    // Example 1: Public API - Get ticker data
    println!("=== BitMart Public API Example ===");
    
    let public_client = PublicRestClient::new();
    
    // Get ticker for BTC_USDT
    let ticker_request = GetTickerRequest {
        symbol: "BTC_USDT".to_string(),
    };
    
    match public_client.get_ticker(ticker_request).await {
        Ok(response) => {
            println!("BTC_USDT Ticker:");
            println!("  Last Price: {}", response.data.last_price);
            println!("  24h Volume: {}", response.data.base_volume);
            println!("  24h Change: {}%", response.data.price_change_percent);
        }
        Err(e) => {
            println!("Error getting ticker: {}", e);
        }
    }

    // Example 2: Private API - Submit an order (requires credentials)
    println!("\n=== BitMart Private API Example ===");
    
    // NOTE: These are example credentials - replace with your actual API credentials
    // You should never hardcode credentials in production code
    let api_key = std::env::var("BITMART_API_KEY")
        .unwrap_or_else(|_| "your_api_key_here".to_string());
    let api_secret = std::env::var("BITMART_API_SECRET")
        .unwrap_or_else(|_| "your_api_secret_here".to_string());
    let memo = std::env::var("BITMART_MEMO")
        .unwrap_or_else(|_| "your_memo_here".to_string());

    if api_key == "your_api_key_here" {
        println!("⚠️  Private API example skipped - no credentials provided");
        println!("   Set BITMART_API_KEY, BITMART_API_SECRET, and BITMART_MEMO environment variables to test private endpoints");
        return Ok(());
    }

    let private_client = PrivateRestClient::new(api_key, api_secret, memo)?;
    
    // Example order request (this is a test order - adjust parameters as needed)
    let order_request = SubmitOrderRequest {
        symbol: "BTC_USDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        client_order_id: Some("example_order_123".to_string()),
        stp_mode: Some(StpMode::None),
        size: Some("0.001".to_string()),    // 0.001 BTC
        price: Some("30000.00".to_string()), // $30,000 USD (example price)
        notional: None,
    };

    println!("Submitting example order:");
    println!("  Symbol: {}", order_request.symbol);
    println!("  Side: {:?}", order_request.side);
    println!("  Type: {:?}", order_request.order_type);
    println!("  Size: {:?}", order_request.size);
    println!("  Price: {:?}", order_request.price);

    // NOTE: This will actually submit an order if credentials are valid
    // Comment out this section if you don't want to submit a real order
    /*
    match private_client.submit_order(order_request).await {
        Ok(response) => {
            println!("Order submitted successfully!");
            println!("  Order ID: {}", response.order_id);
        }
        Err(e) => {
            println!("Error submitting order: {}", e);
        }
    }
    */
    
    println!("Order submission commented out for safety - uncomment to test with real credentials");

    Ok(())
}
