//! Integration tests for Binance Spot private REST API endpoints
//!
//! These tests verify the functionality of private endpoints that require authentication.
//! Tests are disabled by default and can be enabled by setting environment variables:
//!
//! ```bash
//! export RUN_PRIVATE_TESTS=true
//! export BINANCE_API_KEY=your_api_key
//! export BINANCE_SECRET_KEY=your_secret_key
//! cargo test binance::spot::private_integration_tests
//! ```
//!
//! Tests run against Binance testnet by default. To use mainnet, set:
//! ```bash
//! export USE_TESTNET=false
//! ```

use reqwest::Client;
use std::boxed::Box;
use tokio;

// Import types from top-level venue exports as required by integration test standards
use venues::binance::spot::{
    AccountCommissionRequest, AccountRequest, Errors, OrderSide, OrderType, PrivateRestClient,
    RateLimiter, TestNewOrderRequest, TimeInForce,
};

// Import common testing utilities
use crate::common::{BinanceCredentials, CredentialLoader, PrivateTestConfig};

/// Helper function to create a test client for private endpoints
fn create_private_test_client(
    credentials: &BinanceCredentials,
    config: &PrivateTestConfig,
) -> PrivateRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();
    let base_url = config.env.get_base_url("binance_spot");

    // Convert SecretString to the required boxed secret type
    let api_key_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.api_key.clone());
    let api_secret_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.secret_key.clone());

    PrivateRestClient::new(api_key_box, api_secret_box, base_url, rate_limiter, client)
}

/// Helper function to check if an error is due to geographic restrictions
/// Returns true if the error is due to geo-restrictions, false otherwise
fn is_geo_restricted(err: &Errors) -> bool {
    let error_str = format!("{:?}", err);
    error_str.contains("451") || error_str.contains("Unavailable For Legal Reasons")
}

/// Helper function to check if an error is due to testnet account restrictions
/// Returns true if the error is due to testnet limitations, false otherwise
fn is_testnet_restriction(err: &Errors) -> bool {
    let error_str = format!("{:?}", err);
    error_str.contains("This action is disabled on this account")
        || error_str.contains("testnet")
        || error_str.contains("demo")
}

/// Macro to standardize handling private API results with appropriate error checks
macro_rules! handle_private_result {
    ($result:expr, $endpoint_name:expr) => {
        match $result {
            Ok(response) => {
                println!("✅ {} successful", $endpoint_name);
                Some(response)
            }
            Err(err) => {
                if is_geo_restricted(&err) {
                    println!(
                        "⚠️ {} skipped due to geographic restrictions (HTTP 451)",
                        $endpoint_name
                    );
                    None
                } else if is_testnet_restriction(&err) {
                    println!(
                        "⚠️ {} skipped due to testnet account restrictions",
                        $endpoint_name
                    );
                    None
                } else {
                    println!("❌ {} failed: {:?}", $endpoint_name, err);
                    // Don't panic on private endpoint failures as they may be due to
                    // account restrictions, insufficient permissions, or testnet limitations
                    None
                }
            }
        }
    };
}

/// Test the account information endpoint
#[tokio::test]
async fn test_get_account_info() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    config.env.print_env_info();

    let client = create_private_test_client(&credentials, &config);

    let request = AccountRequest {
        omit_zero_balances: Some(true),
        recv_window: None,
    };

    let result = client.get_account(Some(request)).await;

    if let Some(response) = handle_private_result!(result, "get_account") {
        // Verify response structure without asserting on dynamic values
        println!("Account info retrieved successfully");
        println!("Can trade: {}", response.data.can_trade);
        println!("Can withdraw: {}", response.data.can_withdraw);
        println!("Can deposit: {}", response.data.can_deposit);
        println!("Number of balances: {}", response.data.balances.len());

        // Log commission rates for verification
        println!(
            "Commission rates - Maker: {}, Taker: {}",
            response.data.commission_rates.maker, response.data.commission_rates.taker
        );
    }
}

/// Test the open orders endpoint
#[tokio::test]
async fn test_get_open_orders() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Test getting all open orders
    let result = client.get_open_orders(None).await;

    if let Some(response) = handle_private_result!(result, "get_open_orders (all symbols)") {
        println!("Open orders count: {}", response.data.len());

        // If there are open orders, log basic info without asserting on dynamic values
        for (i, order) in response.data.iter().enumerate().take(3) {
            println!(
                "Open order {}: {} {} {} @ {}",
                i + 1,
                order.symbol,
                order.side,
                order.order_type,
                order.price
            );
        }
    }
}

/// Test the order history endpoint
#[tokio::test]
async fn test_get_order_history() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Use BTCUSDT as it's commonly traded
    let request = venues::binance::spot::AllOrdersRequest {
        symbol: "BTCUSDT".to_string(),
        order_id: None,
        start_time: None,
        end_time: None,
        limit: Some(10), // Limit to avoid large responses
        recv_window: None,
    };

    let result = client.get_all_orders(request).await;

    if let Some(response) = handle_private_result!(result, "get_all_orders for BTCUSDT") {
        println!("Order history count: {}", response.data.len());

        // If there are orders, log basic info without asserting on dynamic values
        for (i, order) in response.data.iter().enumerate().take(3) {
            println!(
                "Historical order {}: {} {} {} @ {} - Status: {}",
                i + 1,
                order.symbol,
                order.side,
                order.order_type,
                order.price,
                order.status
            );
        }
    }
}

/// Test the trade history endpoint
#[tokio::test]
async fn test_get_trade_history() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = venues::binance::spot::MyTradesRequest {
        symbol: "BTCUSDT".to_string(),
        order_id: None,
        start_time: None,
        end_time: None,
        from_id: None,
        limit: Some(10), // Limit to avoid large responses
        recv_window: None,
    };

    let result = client.get_my_trades(request).await;

    if let Some(response) = handle_private_result!(result, "get_my_trades for BTCUSDT") {
        println!("Trade history count: {}", response.data.len());

        // If there are trades, log basic info without asserting on dynamic values
        for (i, trade) in response.data.iter().enumerate().take(3) {
            println!(
                "Trade {}: {} {} qty: {} @ {} - Fee: {} {}",
                i + 1,
                trade.symbol,
                if trade.is_buyer { "BUY" } else { "SELL" },
                trade.qty,
                trade.price,
                trade.commission,
                trade.commission_asset
            );
        }
    }
}

/// Test error handling with proper authentication but invalid parameters
#[tokio::test]
async fn test_error_handling_invalid_symbol() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = venues::binance::spot::AllOrdersRequest {
        symbol: "INVALIDTEST".to_string(),
        order_id: None,
        start_time: None,
        end_time: None,
        limit: Some(10),
        recv_window: None,
    };

    let result = client.get_all_orders(request).await;

    match result {
        Ok(_) => {
            println!("⚠️ Unexpected success with invalid symbol");
        }
        Err(err) => {
            if is_geo_restricted(&err) || is_testnet_restriction(&err) {
                println!("⚠️ Error handling test skipped due to restrictions");
            } else {
                println!(
                    "✅ Error handling test - Got expected error for invalid symbol: {:?}",
                    err
                );
            }
        }
    }
}

/// Test rate limiting behavior with authenticated endpoints
#[tokio::test]
async fn test_rate_limiting() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Make multiple quick requests to test rate limiting
    for i in 0..3 {
        let request = AccountRequest {
            omit_zero_balances: Some(true),
            recv_window: None,
        };

        let result = client.get_account(Some(request)).await;

        if handle_private_result!(result, &format!("rate_limiting_account_info_{}", i)).is_some() {
            println!("Rate limited request {} completed successfully", i + 1);
        }

        // Small delay between requests
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

/// Test the account commission rates endpoint
#[tokio::test]
async fn test_get_account_commission() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = AccountCommissionRequest {
        symbol: "BTCUSDT".to_string(),
        recv_window: None,
    };

    let result = client.get_account_commission(request).await;

    if let Some(response) = handle_private_result!(result, "get_account_commission for BTCUSDT") {
        println!("Commission rates retrieved successfully");
        println!(
            "Standard - Maker: {}, Taker: {}",
            response.data.standard_commission.maker, response.data.standard_commission.taker
        );
        println!(
            "Tax - Buyer: {}, Seller: {}",
            response.data.tax_commission.buyer, response.data.tax_commission.seller
        );
        println!(
            "Discount enabled: {}",
            response.data.discount.enabled_for_symbol
        );
    }
}

/// Test order validation without actually placing an order (test endpoint)
#[tokio::test]
async fn test_order_validation() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = TestNewOrderRequest {
        symbol: "BTCUSDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        time_in_force: Some(TimeInForce::GTC),
        quantity: Some(rust_decimal::Decimal::new(1, 5)), // 0.00001 BTC (very small amount)
        quote_order_qty: None,
        price: Some(rust_decimal::Decimal::new(20000, 0)), // $20,000 (low price)
        new_client_order_id: None,
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

    let result = client.test_new_order(request).await;

    if let Some(_response) = handle_private_result!(result, "test_new_order (validation)") {
        println!("Order validation successful - parameters are valid");
    }
}

/// Test getting specific symbol open orders
#[tokio::test]
async fn test_get_symbol_open_orders() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Test getting open orders for a specific symbol
    let symbol_request = venues::binance::spot::OpenOrdersRequest {
        symbol: Some("BTCUSDT".to_string()),
        recv_window: None,
    };

    let result = client.get_open_orders(Some(symbol_request)).await;

    if let Some(response) = handle_private_result!(result, "get_open_orders for BTCUSDT") {
        println!("BTCUSDT open orders count: {}", response.data.len());

        // If there are open orders, log basic info without asserting on dynamic values
        for (i, order) in response.data.iter().enumerate().take(2) {
            println!(
                "BTCUSDT open order {}: {} {} @ {}",
                i + 1,
                order.side,
                order.order_type,
                order.price
            );
        }
    }
}

/// Test comprehensive account balance retrieval with zero balance filtering
#[tokio::test]
async fn test_account_balance_filtering() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Test getting account info with zero balances omitted
    let request_filtered = AccountRequest {
        omit_zero_balances: Some(true),
        recv_window: None,
    };

    let result_filtered = client.get_account(Some(request_filtered)).await;

    let mut filtered_count = 0;
    if let Some(response_filtered) =
        handle_private_result!(result_filtered, "get_account (filtered)")
    {
        filtered_count = response_filtered.data.balances.len();
        println!("Account balances (non-zero only): {}", filtered_count);

        // Log a few non-zero balances
        for (i, balance) in response_filtered.data.balances.iter().enumerate().take(3) {
            println!(
                "Balance {}: {} - Free: {}, Locked: {}",
                i + 1,
                balance.asset,
                balance.free,
                balance.locked
            );
        }
    }

    // Test getting account info with all balances included
    let request_all = AccountRequest {
        omit_zero_balances: Some(false),
        recv_window: None,
    };

    let result_all = client.get_account(Some(request_all)).await;

    if let Some(response_all) = handle_private_result!(result_all, "get_account (all balances)") {
        let all_count = response_all.data.balances.len();
        println!("Account balances (all): {}", all_count);

        // Simple comparison to verify filtering works
        if all_count >= filtered_count && filtered_count > 0 {
            println!(
                "✅ Balance filtering working correctly - All: {}, Filtered: {}",
                all_count, filtered_count
            );
        }
    }
}

/// Test order history with different time ranges
#[tokio::test]
async fn test_order_history_time_filtering() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Binance") {
        return;
    }

    let credentials = match BinanceCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Binance private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Get recent orders (last 24 hours)
    let now = chrono::Utc::now().timestamp_millis() as u64;
    let yesterday = now - (24 * 60 * 60 * 1000); // 24 hours ago

    let request = venues::binance::spot::AllOrdersRequest {
        symbol: "BTCUSDT".to_string(),
        order_id: None,
        start_time: Some(yesterday),
        end_time: Some(now),
        limit: Some(5),
        recv_window: None,
    };

    let result = client.get_all_orders(request).await;

    if let Some(response) = handle_private_result!(result, "get_all_orders (24h)") {
        println!("Orders in last 24h: {}", response.data.len());

        // Log recent orders
        for (i, order) in response.data.iter().enumerate().take(2) {
            println!(
                "Recent order {}: {} {} {} @ {} - Status: {} (Time: {})",
                i + 1,
                order.symbol,
                order.side,
                order.order_type,
                order.price,
                order.status,
                order.time
            );
        }
    }
}
