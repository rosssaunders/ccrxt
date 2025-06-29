//! Example demonstrating how to place and manage orders on Bullish Exchange
//!
//! This example shows how to:
//! 1. Create a private REST client with API credentials
//! 2. Get trading account information
//! 3. Place a limit order
//! 4. Query order status
//! 5. Get trade history
//!
//! Note: This example requires valid API credentials to run successfully.
//! Replace the placeholder credentials with your actual API key and secret.

use rest::secrets::SecretString;
use venues::bullish::{private::RestClient, RateLimiter};
use venues::bullish::{CreateOrderRequest, GetOrdersParams, OrderSide, OrderType, TimeInForce};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ⚠️ Replace these with your actual API credentials
    // Never hardcode credentials in production code!
    let api_key = SecretString::new("your_api_key_here".to_string());
    let api_secret = SecretString::new("your_api_secret_here".to_string());

    // Create HTTP client and rate limiter
    let http_client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();

    // Create private REST client
    let mut client = RestClient::new(
        Box::new(api_key),
        Box::new(api_secret),
        "https://api.exchange.bullish.com",
        http_client,
        rate_limiter,
    );

    println!("🚀 Connecting to Bullish Exchange...");

    // Get trading accounts
    let accounts_response = client.get_trading_accounts().await?;
    println!("📊 Found {} trading accounts", accounts_response.data.len());

    if accounts_response.data.is_empty() {
        println!("❌ No trading accounts found. Please check your API credentials.");
        return Ok(());
    }

    let account = &accounts_response.data[0];
    println!(
        "Using account: {} (Primary: {})",
        account.trading_account_id, account.is_primary_account
    );

    // Example: Place a limit order to buy 0.001 BTC at $30,000
    println!("\n📝 Placing a limit buy order...");
    let order_request = CreateOrderRequest {
        command_type: "V3CreateOrder".to_string(),
        client_order_id: format!("example_{}", chrono::Utc::now().timestamp_millis()),
        symbol: "BTCUSDC".to_string(),
        order_type: OrderType::Limit,
        side: OrderSide::Buy,
        price: Some("30000.00".to_string()),
        stop_price: None,
        quantity: "0.001".to_string(),
        quote_amount: None,
        time_in_force: TimeInForce::Gtc,
        allow_borrow: false,
        trading_account_id: account.trading_account_id.clone(),
    };

    match client.create_order(order_request).await {
        Ok(response) => {
            println!("✅ Order placed successfully!");
            println!("  Order ID: {}", response.order_id);
            println!("  Client Order ID: {}", response.client_order_id);
            println!("  Message: {}", response.message);

            // Query the order status
            println!("\n🔍 Checking order status...");
            match client
                .get_order(&response.order_id, &account.trading_account_id)
                .await
            {
                Ok(order) => {
                    println!("Order Status:");
                    println!("  Status: {:?}", order.status);
                    println!("  Price: {}", order.price);
                    println!("  Quantity: {}", order.quantity);
                    println!("  Filled: {}", order.quantity_filled);
                    println!("  Side: {:?}", order.side);
                    println!("  Type: {:?}", order.order_type);
                }
                Err(e) => println!("❌ Failed to get order status: {}", e),
            }
        }
        Err(e) => {
            println!("❌ Failed to place order: {}", e);
            println!("This might be due to:");
            println!("  - Invalid API credentials");
            println!("  - Insufficient balance");
            println!("  - Market restrictions");
            println!("  - Invalid order parameters");
        }
    }

    // Example: Get recent orders
    println!("\n📋 Getting recent orders...");
    let orders_params = GetOrdersParams {
        trading_account_id: account.trading_account_id.clone(),
        symbol: Some("BTCUSDC".to_string()),
        ..Default::default()
    };

    match client.get_orders(orders_params).await {
        Ok(orders) => {
            println!("Found {} orders", orders.len());
            for (i, order) in orders.iter().take(5).enumerate() {
                println!(
                    "{}. {} {} {} @ {} - Status: {:?}",
                    i + 1,
                    order.side as u8,
                    order.quantity,
                    order.symbol,
                    order.price,
                    order.status
                );
            }
        }
        Err(e) => println!("❌ Failed to get orders: {}", e),
    }

    println!("\n✨ Example completed!");
    Ok(())
}
