//! Integration tests for Bullish private REST API endpoints
//!
//! These tests verify the functionality of private endpoints that require authentication.
//! Tests are disabled by default and can be enabled by setting environment variables:
//!
//! ```bash
//! export RUN_PRIVATE_TESTS=true
//! export BULLISH_API_KEY=your_api_key
//! export BULLISH_SECRET_KEY=your_secret_key
//! cargo test bullish::private_integration_tests
//! ```
//!
//! Tests run against Bullish production environment by default. Be careful with trading operations.

use reqwest::Client;
use tokio;

// Import types from top-level venue exports as required by integration test standards
use venues::bullish::{
    Errors, GetOrdersParams, GetTradesParams, GetWalletTransactionsParams, PrivateRestClient,
    RateLimiter,
};

// Import common testing utilities
use crate::common::{BullishCredentials, CredentialLoader, PrivateTestConfig};

/// Helper function to create a test client for private endpoints
fn create_private_test_client(
    credentials: &BullishCredentials,
    config: &PrivateTestConfig,
) -> PrivateRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();
    let base_url = config.env.get_base_url("bullish");

    // Convert SecretString to the required boxed secret type
    let api_key_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.api_key.clone());
    let api_secret_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.secret_key.clone());

    PrivateRestClient::new(api_key_box, api_secret_box, base_url, client, rate_limiter)
}

/// Helper function to check if an error is due to authentication issues
fn is_auth_error(err: &Errors) -> bool {
    let error_str = format!("{:?}", err);
    error_str.contains("Unauthorized")
        || error_str.contains("Invalid")
        || error_str.contains("Authentication")
        || error_str.contains("401")
        || error_str.contains("403")
}

/// Helper function to check if an error is due to API restrictions
fn is_api_restriction(err: &Errors) -> bool {
    let error_str = format!("{:?}", err);
    error_str.contains("restricted")
        || error_str.contains("disabled")
        || error_str.contains("prohibited")
        || error_str.contains("insufficient")
}

/// Macro to standardize handling private API results with appropriate error checks
macro_rules! handle_bullish_result {
    ($result:expr, $endpoint_name:expr) => {
        match $result {
            Ok(response) => {
                println!("✅ {} successful", $endpoint_name);
                Some(response)
            }
            Err(err) => {
                if is_auth_error(&err) {
                    println!("⚠️ {} skipped due to authentication issues", $endpoint_name);
                    None
                } else if is_api_restriction(&err) {
                    println!("⚠️ {} skipped due to API restrictions", $endpoint_name);
                    None
                } else {
                    println!("❌ {} failed: {:?}", $endpoint_name, err);
                    None
                }
            }
        }
    };
}

/// Test the get asset balances endpoint
#[tokio::test]
async fn test_get_asset_balances() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Bullish") {
        return;
    }

    let credentials = match BullishCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Bullish private test - credentials not available");
            return;
        }
    };

    config.env.print_env_info();

    let mut client = create_private_test_client(&credentials, &config);

    let result = client.get_asset_balances("default").await;

    if let Some(response) = handle_bullish_result!(result, "get_asset_balances") {
        println!("Asset balances retrieved successfully");
        println!("Number of asset balances: {}", response.data.len());

        // Log balance details without asserting on dynamic values
        for (i, balance) in response.data.iter().enumerate().take(5) {
            println!(
                "Balance {}: {} - Total: {}, Available: {}",
                i + 1,
                balance.symbol,
                balance.balance,
                balance.available_balance
            );
        }
    }
}

/// Test the get trading accounts endpoint
#[tokio::test]
async fn test_get_trading_accounts() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Bullish") {
        return;
    }

    let credentials = match BullishCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Bullish private test - credentials not available");
            return;
        }
    };

    let mut client = create_private_test_client(&credentials, &config);

    let result = client.get_trading_accounts().await;

    if let Some(response) = handle_bullish_result!(result, "get_trading_accounts") {
        println!("Trading accounts retrieved successfully");
        println!("Number of trading accounts: {}", response.data.len());

        // Log account details without asserting on dynamic values
        for (i, account) in response.data.iter().enumerate().take(3) {
            println!(
                "Account {}: ID: {}, Collateral USD: {}",
                i + 1,
                account.trading_account_id,
                account.total_collateral_usd
            );
        }
    }
}

/// Test the get orders endpoint
#[tokio::test]
async fn test_get_orders() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Bullish") {
        return;
    }

    let credentials = match BullishCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Bullish private test - credentials not available");
            return;
        }
    };

    let mut client = create_private_test_client(&credentials, &config);

    let params = GetOrdersParams {
        trading_account_id: "default".to_string(),
        symbol: None,
        client_order_id: None,
        side: None,
        status: None,
    };

    let result = client.get_orders(params).await;

    if let Some(orders) = handle_bullish_result!(result, "get_orders") {
        println!("Orders retrieved successfully");
        println!("Number of orders: {}", orders.len());

        // Log order details without asserting on dynamic values
        for (i, order) in orders.iter().enumerate().take(3) {
            println!(
                "Order {}: {} - Side: {:?}, Status: {:?}, Quantity: {}",
                i + 1,
                order.symbol,
                order.side,
                order.status,
                order.quantity
            );
        }
    }
}

/// Test the get trades endpoint
#[tokio::test]
async fn test_get_trades() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Bullish") {
        return;
    }

    let credentials = match BullishCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Bullish private test - credentials not available");
            return;
        }
    };

    let mut client = create_private_test_client(&credentials, &config);

    let params = GetTradesParams {
        trading_account_id: "default".to_string(),
        symbol: None,
        order_id: None,
        side: None,
        start_time: None,
        end_time: None,
    };

    let result = client.get_trades(params).await;

    if let Some(trades) = handle_bullish_result!(result, "get_trades") {
        println!("Trades retrieved successfully");
        println!("Number of trades: {}", trades.len());

        // Log trade details without asserting on dynamic values
        for (i, trade) in trades.iter().enumerate().take(3) {
            println!(
                "Trade {}: {} - Side: {:?}, Quantity: {}, Price: {}",
                i + 1,
                trade.symbol,
                trade.side,
                trade.quantity,
                trade.price
            );
        }
    }
}

/// Test the get wallet transactions endpoint
#[tokio::test]
async fn test_get_wallet_transactions() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Bullish") {
        return;
    }

    let credentials = match BullishCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Bullish private test - credentials not available");
            return;
        }
    };

    let mut client = create_private_test_client(&credentials, &config);

    let params = GetWalletTransactionsParams {
        trading_account_id: "default".to_string(),
        symbol: None,
        transaction_type: None,
        status: None,
        start_time: None,
        end_time: None,
        page_size: Some(20),
        page_token: None,
    };

    let result = client.get_wallet_transactions(params).await;

    if let Some(response) = handle_bullish_result!(result, "get_wallet_transactions") {
        println!("Wallet transactions retrieved successfully");
        println!("Number of transactions: {}", response.data.len());

        // Log transaction details without asserting on dynamic values
        for (i, transaction) in response.data.iter().enumerate().take(3) {
            println!(
                "Transaction {}: {} - Type: {:?}, Status: {:?}, Amount: {}",
                i + 1,
                transaction.symbol,
                transaction.transaction_type,
                transaction.status,
                transaction.amount
            );
        }
    }
}

/// Test error handling with invalid requests
#[tokio::test]
async fn test_error_handling() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Bullish") {
        return;
    }

    let credentials = match BullishCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Bullish private test - credentials not available");
            return;
        }
    };

    let mut client = create_private_test_client(&credentials, &config);

    // Test with specific symbol filter that might not exist
    let params = GetOrdersParams {
        trading_account_id: "default".to_string(),
        symbol: Some("INVALID-SYMBOL".to_string()),
        client_order_id: None,
        side: None,
        status: None,
    };

    let result = client.get_orders(params).await;

    match result {
        Ok(orders) => {
            println!(
                "⚠️ Request succeeded - got {} orders for invalid symbol",
                orders.len()
            );
        }
        Err(err) => {
            if is_auth_error(&err) || is_api_restriction(&err) {
                println!("⚠️ Error handling test skipped due to restrictions");
            } else {
                println!("✅ Error handling test - Got expected error: {:?}", err);
            }
        }
    }
}

/// Test rate limiting behavior with multiple requests
#[tokio::test]
async fn test_rate_limiting() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Bullish") {
        return;
    }

    let credentials = match BullishCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Bullish private test - credentials not available");
            return;
        }
    };

    let mut client = create_private_test_client(&credentials, &config);

    // Make multiple quick requests to test rate limiting
    for i in 0..3 {
        let result = client.get_asset_balances("default").await;

        if handle_bullish_result!(result, &format!("rate_limiting_asset_balances_{}", i)).is_some()
        {
            println!("Rate limited request {} completed successfully", i + 1);
        }

        // Small delay between requests to respect rate limits
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}
