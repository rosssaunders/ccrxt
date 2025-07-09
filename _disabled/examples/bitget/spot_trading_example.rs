//! Bitget Spot Trading API Example
//!
//! This example demonstrates how to use the Bitget spot trading endpoints
//! to place orders, check order status, cancel orders, and retrieve trading history.
//!
//! To run this example:
//! ```bash
//! cargo run --example bitget_spot_trading_example
//! ```

use std::env;
use venues::bitget::private::rest::{RestClient, *};
use venues::bitget::rate_limit::RateLimiter;
use venues::bitget::enums::*;
use rest::secrets::PlainTextSecret;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::init();

    // Get API credentials from environment variables
    let api_key = env::var("BITGET_API_KEY")
        .expect("BITGET_API_KEY environment variable not set");
    let api_secret = env::var("BITGET_API_SECRET")
        .expect("BITGET_API_SECRET environment variable not set");
    let api_passphrase = env::var("BITGET_API_PASSPHRASE")
        .expect("BITGET_API_PASSPHRASE environment variable not set");

    println!("🚀 Bitget Spot Trading Example");
    println!("==============================");

    // Create the REST client
    let client = RestClient::new(
        Box::new(PlainTextSecret::new(api_key)),
        Box::new(PlainTextSecret::new(api_secret)),
        Box::new(PlainTextSecret::new(api_passphrase)),
        "https://api.bitget.com",
        RateLimiter::default(),
        reqwest::Client::new(),
    );

    // Example 1: Get account assets
    println!("\n📊 Getting account assets...");
    match client.get_account_assets(GetAccountAssetsRequest::new()).await {
        Ok(response) => {
            println!("✅ Found {} assets in account", response.data.assets.len());
            for asset in response.data.assets.iter().take(3) {
                println!("   - {}: Available: {}, Frozen: {}", 
                    asset.coin_name, asset.available, asset.frozen);
            }
        }
        Err(e) => println!("❌ Failed to get account assets: {}", e),
    }

    // Example 2: Place a spot order (market buy)
    println!("\n🛒 Placing a small market buy order...");
    let place_order_request = PlaceOrderRequest::new()
        .symbol("BTCUSDT")
        .side(OrderSide::Buy)
        .order_type(OrderType::Market)
        .force(TimeInForce::IOC)
        .size("10"); // $10 worth of BTC

    match client.place_order(place_order_request).await {
        Ok(response) => {
            let order_id = &response.data.order_id;
            println!("✅ Order placed successfully! Order ID: {}", order_id);
            
            // Example 3: Get order information
            println!("\n🔍 Getting order information...");
            let order_info_request = GetOrderInfoRequest::new()
                .symbol("BTCUSDT")
                .order_id(order_id);
                
            match client.get_order_info(order_info_request).await {
                Ok(order_response) => {
                    let order = &order_response.data[0];
                    println!("✅ Order Status: {}", order.status);
                    println!("   Size: {}, Filled: {}", order.size, order.fill_quantity);
                    println!("   Price: {}, Average Fill Price: {}", 
                        order.price, order.price_avg);
                }
                Err(e) => println!("❌ Failed to get order info: {}", e),
            }
        }
        Err(e) => {
            println!("❌ Failed to place order: {}", e);
            println!("   This might be expected in demo mode or with insufficient balance");
        }
    }

    // Example 4: Get current unfilled orders
    println!("\n📋 Getting current unfilled orders...");
    match client.get_current_orders(GetCurrentOrdersRequest::new()).await {
        Ok(response) => {
            println!("✅ Found {} unfilled orders", response.data.len());
            for order in response.data.iter().take(3) {
                println!("   - {}: {} {} at {}", 
                    order.order_id, order.side, order.symbol, order.price);
            }
        }
        Err(e) => println!("❌ Failed to get current orders: {}", e),
    }

    // Example 5: Get order history
    println!("\n📈 Getting order history...");
    let history_request = GetOrderHistoryRequest::new()
        .symbol("BTCUSDT")
        .limit(5);

    match client.get_order_history(history_request).await {
        Ok(response) => {
            println!("✅ Found {} historical orders", response.data.len());
            for order in response.data.iter().take(3) {
                println!("   - {}: {} {} - Status: {}", 
                    order.order_id, order.side, order.symbol, order.status);
            }
        }
        Err(e) => println!("❌ Failed to get order history: {}", e),
    }

    // Example 6: Get recent fills (trades)
    println!("\n💱 Getting recent fills...");
    let fills_request = GetFillsRequest::new()
        .symbol("BTCUSDT")
        .limit(5);

    match client.get_fills(fills_request).await {
        Ok(response) => {
            println!("✅ Found {} recent fills", response.data.len());
            for fill in response.data.iter().take(3) {
                println!("   - Trade ID {}: {} {} at {} (Fee: {} {})", 
                    fill.trade_id, fill.side, fill.symbol, fill.price,
                    fill.fee_amount, fill.fee_currency);
            }
        }
        Err(e) => println!("❌ Failed to get fills: {}", e),
    }

    // Example 7: Cancel an order (demo)
    println!("\n❌ Demonstrating order cancellation...");
    println!("   (This would cancel an order if you had an active order ID)");
    
    // Uncomment and modify this section if you have an active order to cancel:
    /*
    let cancel_request = CancelOrderRequest::new()
        .symbol("BTCUSDT")
        .order_id("your_order_id_here");
        
    match client.cancel_order(cancel_request).await {
        Ok(response) => {
            println!("✅ Order cancelled successfully! Order ID: {}", response.data.order_id);
        }
        Err(e) => println!("❌ Failed to cancel order: {}", e),
    }
    */

    println!("\n🎉 Bitget Spot Trading Example completed!");
    println!("   Remember to set your API credentials in environment variables:");
    println!("   - BITGET_API_KEY");
    println!("   - BITGET_API_SECRET");  
    println!("   - BITGET_API_PASSPHRASE");

    Ok(())
}
