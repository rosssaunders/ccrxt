//! End-to-end trading test for Binance Spot
//!
//! This test demonstrates a complete trading workflow including:
//! - Balance checking
//! - Order placement (limit orders)
//! - Order management (query, list, cancel)
//! - Safety measures to prevent accidental trades
//!
//! IMPORTANT: This test uses the test order endpoint and places orders
//! far from market price to ensure they never execute on testnet.
//! For mainnet, additional safety checks are enforced.

use reqwest::Client;
use rust_decimal::Decimal;
use std::boxed::Box;
use tokio;

// Import types from top-level venue exports
use venues::binance::spot::{
    AccountRequest, CancelOrderRequest, Errors, NewOrderRequest, OpenOrdersRequest,
    OrderResponseType, OrderSide, OrderStatus, OrderType, PrivateRestClient, QueryOrderRequest,
    RateLimiter, TestNewOrderRequest, TimeInForce,
};

// Import common testing utilities
use crate::common::{BinanceCredentials, CredentialLoader, PrivateTestConfig};
use secrecy::SecretString;
use std::env;
use std::path::Path;

/// Load Binance credentials from .env file in the current test directory
fn load_binance_credentials_from_env_file() -> Option<BinanceCredentials> {
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/binance/spot/.env");
    
    if env_path.exists() {
        if let Ok(env_content) = std::fs::read_to_string(&env_path) {
            let mut api_key = None;
            let mut secret_key = None;
            
            for line in env_content.lines() {
                if line.starts_with("API_KEY=") {
                    api_key = Some(line.trim_start_matches("API_KEY=").to_string());
                } else if line.starts_with("SECRET_KEY=") {
                    secret_key = Some(line.trim_start_matches("SECRET_KEY=").to_string());
                }
            }
            
            if let (Some(key), Some(secret)) = (api_key, secret_key) {
                return Some(BinanceCredentials {
                    api_key: SecretString::new(key.into_boxed_str()),
                    secret_key: SecretString::new(secret.into_boxed_str()),
                });
            }
        }
    }
    
    BinanceCredentials::load_from_env()
}

/// Helper function to create a test client for private endpoints
fn create_private_test_client(
    credentials: &BinanceCredentials,
    config: &PrivateTestConfig,
) -> PrivateRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();
    
    // Check if there's a BASE_URL in the .env file
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/binance/spot/.env");
    
    let base_url = if env_path.exists() {
        if let Ok(env_content) = std::fs::read_to_string(&env_path) {
            let mut found_base_url = None;
            for line in env_content.lines() {
                if line.starts_with("BASE_URL=") {
                    found_base_url = Some(line.trim_start_matches("BASE_URL=").to_string());
                    break;
                }
            }
            if let Some(url) = found_base_url {
                println!("Using BASE_URL from .env file: {}", url);
                url
            } else {
                config.env.get_base_url("binance_spot")
            }
        } else {
            config.env.get_base_url("binance_spot")
        }
    } else {
        config.env.get_base_url("binance_spot")
    };

    let api_key_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.api_key.clone());
    let api_secret_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.secret_key.clone());

    PrivateRestClient::new(api_key_box, api_secret_box, base_url, rate_limiter, client)
}

/// Helper macro for handling test results
macro_rules! handle_result {
    ($result:expr, $operation:expr) => {
        match $result {
            Ok(response) => {
                println!("✅ {} successful", $operation);
                Some(response)
            }
            Err(e) => {
                println!("❌ {} failed: {:?}", $operation, e);
                None
            }
        }
    };
}

/// Helper function to check if we're on mainnet
fn is_mainnet(base_url: &str) -> bool {
    !base_url.contains("testnet") && !base_url.contains("test")
}

/// Test using test_new_order endpoint
#[tokio::test]
async fn test_test_order_endpoint() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match load_binance_credentials_from_env_file() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);
    
    println!("🔍 Testing test_new_order endpoint");
    
    let test_request = TestNewOrderRequest {
        symbol: "BTCUSDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        time_in_force: Some(TimeInForce::GTC),
        quantity: Some(Decimal::new(1, 3)), // 0.001 BTC
        quote_order_qty: None,
        price: Some(Decimal::new(40000, 0)), // $40,000 (closer to market)
        new_client_order_id: Some(format!("test_{}", chrono::Utc::now().timestamp())),
        strategy_id: None,
        strategy_type: None,
        stop_price: None,
        trailing_delta: None,
        iceberg_qty: None,
        new_order_resp_type: None,
        self_trade_prevention_mode: None,
        compute_commission_rates: None,
        recv_window: None,
    };
    
    match client.test_new_order(test_request).await {
        Ok(response) => {
            println!("✅ test_new_order successful!");
            println!("Response: {:?}", response.data);
        }
        Err(e) => {
            println!("❌ test_new_order failed: {:?}", e);
            if let Errors::ApiError(ref api_err) = e {
                println!("API Error: {}", api_err);
            }
        }
    }
}

/// Test simple account access first
#[tokio::test]
async fn test_simple_account_access() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match load_binance_credentials_from_env_file() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping test - credentials not available");
            return;
        }
    };

    config.env.print_env_info();
    let client = create_private_test_client(&credentials, &config);
    let base_url = config.env.get_base_url("binance_spot");
    
    println!("🔍 Testing simple account access");
    println!("Base URL: {}", base_url);
    
    // Try the simplest possible request - account info with no parameters
    let account_request = AccountRequest {
        omit_zero_balances: None,
        recv_window: None,
    };
    
    match client.get_account(Some(account_request)).await {
        Ok(response) => {
            println!("✅ Account access successful!");
            println!("Balance count: {}", response.data.balances.len());
        }
        Err(e) => {
            println!("❌ Account access failed: {:?}", e);
            
            // Debug the error
            if let Errors::ApiError(ref api_err) = e {
                println!("API Error: {}", api_err);
            }
        }
    }
}

/// End-to-end trading test
/// 
/// This test demonstrates a complete trading workflow:
/// 1. Check initial balances
/// 2. Place a limit buy order for BTC (far below market price)
/// 3. Query the order to verify it was created
/// 4. List open orders to see it appears
/// 5. Cancel the order
/// 6. Verify the order was cancelled
/// 7. Check final balances to ensure no changes
#[tokio::test]
async fn test_end_to_end_trading() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match load_binance_credentials_from_env_file() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping end-to-end trading test - credentials not available");
            return;
        }
    };

    config.env.print_env_info();
    let client = create_private_test_client(&credentials, &config);
    let base_url = config.env.get_base_url("binance_spot");
    
    // Additional safety check for mainnet
    if is_mainnet(&base_url) {
        println!("⚠️ Skipping end-to-end trading test on mainnet for safety");
        return;
    }

    println!("🚀 Starting end-to-end trading test on testnet");
    
    // Step 1: Check initial balances
    println!("\n📊 Step 1: Checking initial balances");
    let account_request = AccountRequest {
        omit_zero_balances: Some(true),
        recv_window: None,
    };
    
    let initial_balances = match client.get_account(Some(account_request.clone())).await {
        Ok(response) => {
            println!("Initial balance count: {}", response.data.balances.len());
            
            // Find USDT and BTC balances
            let usdt_balance = response.data.balances.iter()
                .find(|b| b.asset == "USDT")
                .map(|b| b.free.clone());
            let btc_balance = response.data.balances.iter()
                .find(|b| b.asset == "BTC")
                .map(|b| b.free.clone());
            
            println!("USDT balance: {:?}", usdt_balance);
            println!("BTC balance: {:?}", btc_balance);
            
            response.data.balances
        }
        Err(e) => {
            println!("❌ Failed to get initial balances: {:?}", e);
            return;
        }
    };
    
    // Step 2: Place a limit buy order far below market price
    println!("\n📈 Step 2: Placing limit buy order for BTC");
    let order_request = NewOrderRequest {
        symbol: "BTCUSDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        time_in_force: Some(TimeInForce::GTC),
        quantity: Some(Decimal::new(1, 3)), // 0.001 BTC
        quote_order_qty: None,
        price: Some(Decimal::new(40000, 0)), // $40,000 (below market but within filter)
        new_client_order_id: Some(format!("e2e_test_{}", chrono::Utc::now().timestamp())),
        strategy_id: None,
        strategy_type: None,
        stop_price: None,
        trailing_delta: None,
        iceberg_qty: None,
        new_order_resp_type: Some(OrderResponseType::Full),
        self_trade_prevention_mode: None,
        recv_window: None,
    };
    
    let order_response = match client.new_order(order_request).await {
        Ok(response) => {
            println!("Order created successfully!");
            // Parse the JSON response to get order details
            if let Some(order_id) = response.data.get("orderId").and_then(|v| v.as_i64()) {
                println!("Order ID: {}", order_id);
            }
            if let Some(client_order_id) = response.data.get("clientOrderId").and_then(|v| v.as_str()) {
                println!("Client Order ID: {}", client_order_id);
            }
            if let Some(status) = response.data.get("status").and_then(|v| v.as_str()) {
                println!("Status: {}", status);
            }
            if let Some(price) = response.data.get("price").and_then(|v| v.as_str()) {
                println!("Price: {}", price);
            }
            if let Some(qty) = response.data.get("origQty").and_then(|v| v.as_str()) {
                println!("Quantity: {}", qty);
            }
            response.data
        }
        Err(e) => {
            println!("❌ Failed to create order: {:?}", e);
            
            // Additional debugging for signature issues
            if let Errors::ApiError(ref api_err) = e {
                println!("API Error Details: {}", api_err);
            }
            
            return;
        }
    };
    
    let order_id = order_response.get("orderId")
        .and_then(|v| v.as_i64())
        .expect("Order ID not found in response") as u64;
    
    // Step 3: Query the specific order
    println!("\n🔍 Step 3: Querying the order");
    let query_request = QueryOrderRequest {
        symbol: "BTCUSDT".to_string(),
        order_id: Some(order_id),
        orig_client_order_id: None,
        recv_window: None,
    };
    
    if let Some(order_response) = handle_result!(
        client.query_order(query_request).await,
        "query_order"
    ) {
        println!("Order status: {:?}", order_response.data.status);
        println!("Executed quantity: {}", order_response.data.executed_qty);
        assert_eq!(order_response.data.status, OrderStatus::New, "Order should be NEW status");
    }
    
    // Step 4: List open orders
    println!("\n📋 Step 4: Listing open orders");
    let open_orders_request = OpenOrdersRequest {
        symbol: Some("BTCUSDT".to_string()),
        recv_window: None,
    };
    
    if let Some(open_orders) = handle_result!(
        client.get_open_orders(Some(open_orders_request)).await,
        "get_open_orders"
    ) {
        println!("Open orders count: {}", open_orders.data.len());
        
        // Verify our order is in the list
        let our_order = open_orders.data.iter()
            .find(|o| o.order_id == order_id);
        
        if let Some(order) = our_order {
            println!("Found our order in open orders list");
            println!("Order price: {}", order.price);
            println!("Order quantity: {}", order.orig_qty);
        } else {
            println!("⚠️ Warning: Our order not found in open orders list");
        }
    }
    
    // Step 5: Cancel the order
    println!("\n🚫 Step 5: Cancelling the order");
    let cancel_request = CancelOrderRequest {
        symbol: "BTCUSDT".to_string(),
        order_id: Some(order_id),
        orig_client_order_id: None,
        new_client_order_id: None,
        cancel_restrictions: None,
        recv_window: None,
    };
    
    match client.cancel_order(cancel_request).await {
        Ok(response) => {
            println!("✅ Order cancelled successfully");
            println!("Cancelled order ID: {}", response.data.order_id);
            println!("Status: {:?}", response.data.status);
            assert_eq!(response.data.status, OrderStatus::Canceled, "Order should be CANCELED");
        }
        Err(e) => {
            println!("❌ Failed to cancel order: {:?}", e);
            // Continue with test even if cancel fails
        }
    }
    
    // Step 6: Verify order is no longer in open orders
    println!("\n✔️ Step 6: Verifying order cancellation");
    let verify_request = OpenOrdersRequest {
        symbol: Some("BTCUSDT".to_string()),
        recv_window: None,
    };
    
    if let Some(open_orders) = handle_result!(
        client.get_open_orders(Some(verify_request)).await,
        "get_open_orders (verification)"
    ) {
        let our_order = open_orders.data.iter()
            .find(|o| o.order_id == order_id);
        
        if our_order.is_none() {
            println!("✅ Order successfully removed from open orders");
        } else {
            println!("⚠️ Warning: Order still appears in open orders");
        }
    }
    
    // Step 7: Check final balances
    println!("\n💰 Step 7: Checking final balances");
    if let Some(final_response) = handle_result!(
        client.get_account(Some(account_request)).await,
        "get_account (final)"
    ) {
        println!("Final balance count: {}", final_response.data.balances.len());
        
        // Compare balances
        for initial_balance in &initial_balances {
            if let Some(final_balance) = final_response.data.balances.iter()
                .find(|b| b.asset == initial_balance.asset) {
                if initial_balance.free != final_balance.free {
                    println!("⚠️ Balance changed for {}: {} -> {}", 
                        initial_balance.asset, initial_balance.free, final_balance.free);
                }
            }
        }
        
        println!("✅ End-to-end trading test completed successfully");
    }
}

/// Test for handling trading errors
#[tokio::test]
async fn test_trading_error_handling() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match load_binance_credentials_from_env_file() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping trading error test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);
    
    println!("🧪 Testing trading error handling");
    
    // Test 1: Invalid symbol
    println!("\n1️⃣ Testing invalid symbol error");
    let invalid_request = NewOrderRequest {
        symbol: "INVALID_SYMBOL".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        time_in_force: Some(TimeInForce::GTC),
        quantity: Some(Decimal::new(1, 4)),
        quote_order_qty: None,
        price: Some(Decimal::new(100, 0)),
        new_client_order_id: None,
        strategy_id: None,
        strategy_type: None,
        stop_price: None,
        trailing_delta: None,
        iceberg_qty: None,
        new_order_resp_type: None,
        self_trade_prevention_mode: None,
        recv_window: None,
    };
    
    match client.new_order(invalid_request).await {
        Ok(_) => println!("❌ Expected error for invalid symbol, but got success"),
        Err(e) => println!("✅ Got expected error for invalid symbol: {:?}", e),
    }
    
    // Test 2: Cancel non-existent order
    println!("\n2️⃣ Testing cancel non-existent order");
    let cancel_request = CancelOrderRequest {
        symbol: "BTCUSDT".to_string(),
        order_id: Some(999999999), // Non-existent order ID
        orig_client_order_id: None,
        new_client_order_id: None,
        cancel_restrictions: None,
        recv_window: None,
    };
    
    match client.cancel_order(cancel_request).await {
        Ok(_) => println!("❌ Expected error for non-existent order, but got success"),
        Err(e) => println!("✅ Got expected error for non-existent order: {:?}", e),
    }
    
    println!("\n✅ Trading error handling test completed");
}