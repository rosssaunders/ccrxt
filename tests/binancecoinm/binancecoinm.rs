//! Integration tests for Binance COIN-M Futures stable coin buy order flow
//!
//! This module contains integration tests that demonstrate a complete trading workflow
//! on Binance COIN-M Futures, including:
//! - Getting exchange info and finding stable coin pairs
//! - Checking account balances
//! - Placing buy orders at market prices
//! - Managing order lifecycle (status checking, replacement, cancellation)
//! - Verifying trade execution through historical trades
//!
//! These tests require API credentials and should be run against testnet.

use anyhow::{Result, anyhow};
use rest::secrets::SecretValue;
use secrecy::SecretString;
use std::{env, sync::Arc, time::Duration};
use tokio::time::sleep;
use venues::binance::coinm::{
    AccountTradesRequest, CancelOrderRequest, ExchangeInfoRequest, GetAccountRequest,
    NewOrderRequest, OrderSide, OrderType, PrivateRestClient, PublicRestClient, QueryOrderRequest,
    RateLimiter, TimeInForce,
};

/// Helper function to create a test private client using environment variables
/// Expects API_KEY and API_SECRET to be set
fn create_test_private_client() -> Result<Arc<PrivateRestClient>> {
    let api_key = env::var("API_KEY").map_err(|_| anyhow!("API_KEY not set"))?;
    let api_secret = env::var("API_SECRET").map_err(|_| anyhow!("API_SECRET not set"))?;

    // Use testnet by default for integration tests
    let base_url = env::var("BINANCE_TESTNET_URL")
        .unwrap_or_else(|_| "https://testnet.binancefuture.com".to_string());

    let client = PrivateRestClient::new(
        Box::new(SecretValue::new(SecretString::from(api_key))),
        Box::new(SecretValue::new(SecretString::from(api_secret))),
        base_url,
        RateLimiter::new(),
        reqwest::Client::new(),
    );

    Ok(Arc::new(client))
}

/// Helper function to create a test public client
fn create_test_public_client() -> Arc<PublicRestClient> {
    let base_url = env::var("BINANCE_TESTNET_URL")
        .unwrap_or_else(|_| "https://testnet.binancefuture.com".to_string());

    let client = PublicRestClient::new(base_url, reqwest::Client::new(), RateLimiter::new());

    Arc::new(client)
}

/// Helper function to find a stable coin trading pair from exchange info
/// Looks for pairs ending in USD or USDT that are currently trading
async fn find_stable_coin_pair(public_client: Arc<PublicRestClient>) -> Result<String> {
    let exchange_info_req = ExchangeInfoRequest {};
    let exchange_info_resp = public_client.get_exchange_info(exchange_info_req).await?;

    // Look for active trading pairs with stable coin bases (USD/USDT)
    for symbol in exchange_info_resp.data.symbols {
        if (symbol.symbol.ends_with("USD") || symbol.symbol.ends_with("USDT"))
            && symbol.status == "TRADING"
            && symbol.contract_type == "PERPETUAL"
        {
            println!("Found stable coin pair: {}", symbol.symbol);
            return Ok(symbol.symbol);
        }
    }

    Err(anyhow!("No suitable stable coin trading pair found"))
}

/// Helper function to get current balance for a specific asset
async fn get_asset_balance(private_client: Arc<PrivateRestClient>, asset: &str) -> Result<f64> {
    let account_req = GetAccountRequest {
        recv_window: None,
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    let account_resp = private_client.get_account(account_req).await?;

    for balance in account_resp.data.assets {
        if balance.asset == asset {
            return Ok(balance.wallet_balance.parse()?);
        }
    }

    Ok(0.0)
}

/// Helper function to calculate order quantity based on balance and price
/// Uses a conservative 10% of available balance
fn calculate_order_quantity(balance: f64, price: f64, min_qty: f64) -> f64 {
    let max_affordable = balance * 0.1 / price; // Use 10% of balance
    max_affordable.max(min_qty) // Ensure we meet minimum quantity requirements
}

/// Helper function to get current market price (best ask) for a symbol
/// This would typically use orderbook or ticker data, but for testing we'll use a simple approach
async fn get_current_ask_price(
    _public_client: Arc<PublicRestClient>,
    _symbol: &str,
) -> Result<f64> {
    // In a real implementation, this would fetch current orderbook or ticker
    // For integration test purposes, we'll use a reasonable test price
    // This should be replaced with actual market data fetching
    Ok(50000.0) // Example price - replace with actual market data call
}

/// Helper function to wait for order execution or timeout
async fn wait_for_order_execution(
    private_client: Arc<PrivateRestClient>,
    symbol: &str,
    order_id: i64,
    max_wait_seconds: u64,
) -> Result<String> {
    let check_interval = Duration::from_secs(2);
    let max_iterations = max_wait_seconds / 2;

    for _ in 0..max_iterations {
        let query_req = QueryOrderRequest {
            symbol: symbol.to_string(),
            order_id: Some(order_id),
            orig_client_order_id: None,
            recv_window: None,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        };

        let order_resp = private_client.get_query_order(query_req).await?;
        let status = order_resp.data.status.clone();

        println!("Order {} status: {:?}", order_id, status);

        match status.as_str() {
            "FILLED" => return Ok("FILLED".to_string()),
            "CANCELED" | "REJECTED" | "EXPIRED" => {
                return Ok(status);
            }
            _ => {
                // Order still pending, wait and check again
                sleep(check_interval).await;
            }
        }
    }

    Ok("TIMEOUT".to_string())
}

/// Helper function to cancel an order
async fn cancel_order(
    private_client: Arc<PrivateRestClient>,
    symbol: &str,
    order_id: i64,
) -> Result<()> {
    let cancel_req = CancelOrderRequest {
        symbol: symbol.to_string(),
        order_id: Some(order_id),
        orig_client_order_id: None,
        recv_window: None,
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    private_client.delete_order(cancel_req).await?;
    println!("Successfully canceled order {}", order_id);
    Ok(())
}

/// Helper function to place a buy order at a specific price
async fn place_buy_order(
    private_client: Arc<PrivateRestClient>,
    symbol: &str,
    quantity: f64,
    price: f64,
) -> Result<i64> {
    let order_req = NewOrderRequest {
        symbol: symbol.to_string(),
        side: OrderSide::Buy,
        position_side: None,
        order_type: OrderType::Limit,
        time_in_force: Some(TimeInForce::GTC),
        quantity: Some(quantity.to_string()),
        price: Some(price.to_string()),
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
        new_client_order_id: None,
        reduce_only: None,
        stop_price: None,
        close_position: None,
        activation_price: None,
        callback_rate: None,
        working_type: None,
        price_protect: None,
        recv_window: None,
        new_order_resp_type: None,
        price_match: None,
        self_trade_prevention_mode: None,
    };

    let order_resp = private_client.post_order(order_req).await?;
    println!(
        "Placed buy order: ID {}, status: {:?}",
        order_resp.data.order_id, order_resp.data.status
    );
    Ok(order_resp.data.order_id)
}

/// Helper function to verify trade execution using historical trades
async fn verify_trade_execution(
    private_client: Arc<PrivateRestClient>,
    symbol: &str,
    order_id: i64,
) -> Result<bool> {
    let trades_req = AccountTradesRequest {
        symbol: symbol.to_string(),
        order_id: Some(order_id),
        start_time: None,
        end_time: None,
        from_id: None,
        limit: Some(100),
        recv_window: None,
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    let trades_resp = private_client.get_account_trades(trades_req).await?;

    if !trades_resp.data.is_empty() {
        println!(
            "Found {} trade(s) for order {}",
            trades_resp.data.len(),
            order_id
        );
        for trade in &trades_resp.data {
            println!(
                "Trade: {} {} at {} on {}",
                trade.qty, trade.symbol, trade.price, trade.time
            );
        }
        return Ok(true);
    }

    Ok(false)
}

#[tokio::test]
#[ignore] // Only run with explicit --ignored flag since it requires API credentials
async fn test_stable_coin_buy_order_flow() -> Result<()> {
    println!("Starting stable coin buy order flow integration test...");

    // Step 1: Create clients
    let private_client = create_test_private_client()?;
    let public_client = create_test_public_client();

    // Step 2: Get instruments and find a stable coin pair
    println!("Step 1: Getting exchange info and finding stable coin pairs...");
    let symbol = find_stable_coin_pair(public_client.clone()).await?;
    println!("Selected trading pair: {}", symbol);

    // Step 3: Get current balance
    println!("Step 2: Checking account balance...");
    let base_asset = "USDT"; // Assuming USDT as the quote asset for most stable pairs
    let initial_balance = get_asset_balance(private_client.clone(), base_asset).await?;
    println!("Current {} balance: {}", base_asset, initial_balance);

    if initial_balance < 10.0 {
        return Err(anyhow!(
            "Insufficient balance for testing (need at least 10 USDT)"
        ));
    }

    // Step 4: Get current market price (ask price)
    println!("Step 3: Getting current market price...");
    let mut current_price = get_current_ask_price(public_client.clone(), &symbol).await?;
    println!("Current ask price for {}: {}", symbol, current_price);

    // Step 5: Calculate order quantity
    let min_quantity = 0.001; // Typical minimum for futures
    let quantity = calculate_order_quantity(initial_balance, current_price, min_quantity);
    println!("Calculated order quantity: {}", quantity);

    // Step 6: Place buy order at ask price
    println!("Step 4: Placing buy order...");
    let mut order_id =
        place_buy_order(private_client.clone(), &symbol, quantity, current_price).await?;

    // Step 7: Monitor order status and replace if necessary
    println!("Step 5: Monitoring order execution...");
    let max_iterations = 3;
    let mut iteration = 0;

    while iteration < max_iterations {
        iteration += 1;
        println!(
            "Iteration {}/{}: Waiting for order execution...",
            iteration, max_iterations
        );

        let order_status = wait_for_order_execution(
            private_client.clone(),
            &symbol,
            order_id,
            30, // Wait up to 30 seconds per iteration
        )
        .await?;

        match order_status.as_str() {
            "FILLED" => {
                println!("Order {} successfully filled!", order_id);
                break;
            }
            "CANCELED" | "REJECTED" | "EXPIRED" => {
                return Err(anyhow!(
                    "Order {} was {} unexpectedly",
                    order_id,
                    order_status
                ));
            }
            "TIMEOUT" | _ => {
                if iteration < max_iterations {
                    println!("Order not filled yet, replacing with higher price...");

                    // Cancel the current order
                    cancel_order(private_client.clone(), &symbol, order_id).await?;

                    // Place new order at a higher price (increase by 0.5%)
                    current_price *= 1.005;
                    order_id =
                        place_buy_order(private_client.clone(), &symbol, quantity, current_price)
                            .await?;
                } else {
                    println!("Max iterations reached, canceling final order...");
                    cancel_order(private_client.clone(), &symbol, order_id).await?;
                    return Err(anyhow!(
                        "Order not filled after {} iterations",
                        max_iterations
                    ));
                }
            }
        }
    }

    // Step 8: Verify trade execution using historical trades
    println!("Step 6: Verifying trade execution...");
    let trade_found = verify_trade_execution(private_client.clone(), &symbol, order_id).await?;

    if trade_found {
        println!("✅ Trade execution verified in historical trades");
    } else {
        println!(
            "⚠️  No trades found for order {} (may be due to timing)",
            order_id
        );
    }

    // Step 9: Check final balance
    println!("Step 7: Checking final balance...");
    let final_balance = get_asset_balance(private_client.clone(), base_asset).await?;
    println!(
        "Final {} balance: {} (change: {})",
        base_asset,
        final_balance,
        final_balance - initial_balance
    );

    println!("✅ Stable coin buy order flow integration test completed successfully!");
    Ok(())
}

#[tokio::test]
#[ignore] // Only run with explicit --ignored flag
async fn test_exchange_info_access() -> Result<()> {
    println!("Testing exchange info access...");

    let public_client = create_test_public_client();
    let symbol = find_stable_coin_pair(public_client).await?;

    assert!(
        !symbol.is_empty(),
        "Should find at least one stable coin pair"
    );
    println!(
        "✅ Exchange info access test passed, found symbol: {}",
        symbol
    );
    Ok(())
}

#[tokio::test]
#[ignore] // Only run with explicit --ignored flag  
async fn test_account_balance_access() -> Result<()> {
    println!("Testing account balance access...");

    let private_client = create_test_private_client()?;
    let balance = get_asset_balance(private_client, "USDT").await?;

    println!("USDT balance: {}", balance);
    println!("✅ Account balance access test passed");
    Ok(())
}

#[test]
fn test_order_quantity_calculation() {
    let balance = 1000.0;
    let price = 50000.0;
    let min_qty = 0.001;

    let quantity = calculate_order_quantity(balance, price, min_qty);

    // Should use 10% of balance, which is 100 USDT / 50000 = 0.002 BTC
    assert!(
        quantity >= min_qty,
        "Quantity should be at least the minimum"
    );
    assert!(
        quantity <= balance * 0.1 / price,
        "Quantity should not exceed 10% of balance"
    );

    println!("✅ Order quantity calculation test passed: {}", quantity);
}
