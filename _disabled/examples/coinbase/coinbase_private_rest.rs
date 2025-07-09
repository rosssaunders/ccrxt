//! Example demonstrating Coinbase Exchange private REST API endpoints
//!
//! This example shows how to use the newly implemented private REST endpoints
//! for order management and fill retrieval.

use std::env;

use reqwest::Client;

use rest::secrets::StaticSecret;
use venues::coinbase::{
    RateLimiter,
    enums::{OrderSide, OrderStatus, OrderType},
    private::rest::{
        CancelAllOrdersRequest, CreateOrderRequest, GetFillsRequest, GetOrdersRequest, RestClient,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment variables
    let api_key = env::var("COINBASE_API_KEY").expect("COINBASE_API_KEY must be set");
    let api_secret = env::var("COINBASE_API_SECRET").expect("COINBASE_API_SECRET must be set");
    let api_passphrase =
        env::var("COINBASE_API_PASSPHRASE").expect("COINBASE_API_PASSPHRASE must be set");

    // Use sandbox URL for testing
    let base_url = "https://api-public.sandbox.exchange.coinbase.com";

    // Create the REST client
    let client = RestClient::new(
        Box::new(StaticSecret::new(api_key)),
        Box::new(StaticSecret::new(api_secret)),
        Box::new(StaticSecret::new(api_passphrase)),
        base_url,
        Client::new(),
        RateLimiter::new(),
    );

    println!("=== Coinbase Exchange Private REST API Example ===\n");

    // Example 1: Get all open orders
    println!("1. Getting all open orders...");
    let orders_request = GetOrdersRequest {
        product_id: Some("BTC-USD".to_string()),
        status: Some(vec![OrderStatus::Open]),
        limit: Some(10),
        ..Default::default()
    };

    match client.get_orders(&orders_request).await {
        Ok((orders, pagination)) => {
            println!("✓ Found {} open orders", orders.len());
            if let Some(pagination) = pagination {
                println!(
                    "  Pagination: before={:?}, after={:?}",
                    pagination.before, pagination.after
                );
            }
            for order in &orders {
                println!(
                    "  Order {}: {} {} {} @ ${}",
                    order.id, order.side, order.size, order.product_id, order.price
                );
            }
        }
        Err(e) => println!("✗ Error getting orders: {}", e),
    }

    println!();

    // Example 2: Create a limit order (this will likely fail in sandbox without funds)
    println!("2. Creating a test limit order...");
    let create_order_request = CreateOrderRequest {
        order_type: OrderType::Limit,
        side: OrderSide::Buy,
        product_id: "BTC-USD".to_string(),
        price: Some("30000.00".to_string()), // Low price to avoid accidental execution
        size: Some("0.001".to_string()),     // Small size
        post_only: Some(true),               // Post-only to avoid immediate execution
        ..Default::default()
    };

    match client.create_order(&create_order_request).await {
        Ok(order) => {
            println!("✓ Created order: {}", order.id);
            println!("  Status: {:?}", order.status);
            println!(
                "  Side: {:?}, Size: {}, Price: ${}",
                order.side, order.size, order.price
            );

            // Example 3: Get the specific order we just created
            println!("\n3. Getting the order we just created...");
            match client.get_order(&order.id, &Default::default()).await {
                Ok(fetched_order) => {
                    println!("✓ Fetched order: {}", fetched_order.id);
                    println!("  Status: {:?}", fetched_order.status);
                }
                Err(e) => println!("✗ Error getting order: {}", e),
            }

            // Example 4: Cancel the order we created
            println!("\n4. Canceling the order...");
            match client.cancel_order(&order.id, &Default::default()).await {
                Ok(canceled_id) => {
                    println!("✓ Canceled order: {:?}", canceled_id);
                }
                Err(e) => println!("✗ Error canceling order: {}", e),
            }
        }
        Err(e) => println!("✗ Error creating order: {}", e),
    }

    println!();

    // Example 5: Get recent fills
    println!("5. Getting recent fills...");
    let fills_request = GetFillsRequest {
        product_id: Some("BTC-USD".to_string()),
        limit: Some(5),
        ..Default::default()
    };

    match client.get_fills(&fills_request).await {
        Ok((fills, pagination)) => {
            println!("✓ Found {} recent fills", fills.len());
            if let Some(pagination) = pagination {
                println!(
                    "  Pagination: before={:?}, after={:?}",
                    pagination.before, pagination.after
                );
            }
            for fill in &fills {
                println!(
                    "  Fill {}: {} {} {} @ ${} (fee: ${})",
                    fill.trade_id, fill.side, fill.size, fill.product_id, fill.price, fill.fee
                );
            }
        }
        Err(e) => println!("✗ Error getting fills: {}", e),
    }

    println!();

    // Example 6: Cancel all orders (be careful with this in production!)
    println!("6. Canceling all orders...");
    let cancel_all_request = CancelAllOrdersRequest {
        product_id: Some("BTC-USD".to_string()),
        ..Default::default()
    };

    match client.cancel_all_orders(&cancel_all_request).await {
        Ok(canceled_ids) => {
            println!("✓ Canceled {} orders", canceled_ids.len());
            for id in &canceled_ids {
                println!("  Canceled: {}", id);
            }
        }
        Err(e) => println!("✗ Error canceling all orders: {}", e),
    }

    println!("\n=== Example completed ===");
    println!(
        "Note: Some operations may fail in sandbox mode due to insufficient funds or other restrictions."
    );

    Ok(())
}
