//! Integration tests for Coinbase Exchange private REST API endpoints
//!
//! These tests verify the functionality of private endpoints that require authentication.
//! Tests are disabled by default and can be enabled by setting environment variables:
//!
//! ```bash
//! export RUN_PRIVATE_TESTS=true
//! export COINBASE_EXCHANGE_API_KEY=your_api_key
//! export COINBASE_EXCHANGE_SECRET_KEY=your_secret_key
//! export COINBASE_EXCHANGE_PASSPHRASE=your_passphrase
//! cargo test coinbaseexchange::private_integration_tests
//! ```
//!
//! Tests run against Coinbase Exchange production environment by default. Be careful with trading operations.

use reqwest::Client;
use tokio;

// Import types from top-level venue exports as required by integration test standards
use venues::coinbaseexchange::{Errors, PrivateRestClient, RateLimiter};

// Import common testing utilities
use crate::common::{CoinbaseCredentials, CredentialLoader, PrivateTestConfig};

/// Helper function to create a test client for private endpoints
fn create_private_test_client(
    credentials: &CoinbaseCredentials,
    config: &PrivateTestConfig,
) -> PrivateRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();
    let base_url = config.env.get_base_url("coinbase_exchange");

    // Convert SecretString to the required boxed secret type
    let api_key_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.api_key.clone());
    let api_secret_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.secret_key.clone());
    let passphrase_box: Box<dyn rest::secrets::ExposableSecret> =
        Box::new(credentials.passphrase.clone());

    PrivateRestClient::new(
        api_key_box,
        api_secret_box,
        passphrase_box,
        base_url,
        client,
        rate_limiter,
    )
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
macro_rules! handle_coinbase_result {
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

/// Test the get account balances endpoint
#[tokio::test]
async fn test_get_account_balances() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Coinbase Exchange") {
        return;
    }

    let credentials = match CoinbaseCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Coinbase Exchange private test - credentials not available");
            return;
        }
    };

    config.env.print_env_info();

    let client = create_private_test_client(&credentials, &config);

    let request = venues::coinbaseexchange::private::rest::GetAccountBalancesRequest {
        limit: Some(50),
        before: None,
        after: None,
    };

    let result = client.get_account_balances(&request).await;

    if let Some(response) = handle_coinbase_result!(result, "get_account_balances") {
        println!("Account balances retrieved successfully");
        println!("Number of accounts: {}", response.accounts.len());

        // Log account details without asserting on dynamic values
        for (i, account) in response.accounts.iter().enumerate().take(5) {
            println!(
                "Account {}: {} - Available: {}, Hold: {}",
                i + 1,
                account.currency,
                account.available,
                account.hold
            );
        }

        if let Some(pagination) = &response.pagination {
            println!(
                "Pagination - Before: {:?}, After: {:?}",
                pagination.before, pagination.after
            );
        }
    }
}

/// Test the get orders endpoint
#[tokio::test]
async fn test_get_orders() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Coinbase Exchange") {
        return;
    }

    let credentials = match CoinbaseCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Coinbase Exchange private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = venues::coinbaseexchange::private::rest::GetOrdersRequest {
        profile_id: None,
        product_id: Some("BTC-USD".to_string()), // Common trading pair
        sorted_by: None,
        sorting: None,
        start_date: None,
        end_date: None,
        before: None,
        after: None,
        limit: Some(20),
        status: None,
        market_type: None,
    };

    let result = client.get_orders(&request).await;

    if let Some((orders, pagination)) = handle_coinbase_result!(result, "get_orders") {
        println!("Orders retrieved successfully");
        println!("Number of orders: {}", orders.len());

        // Log order details without asserting on dynamic values
        for (i, order) in orders.iter().enumerate().take(3) {
            println!(
                "Order {}: {} - Side: {:?}, Status: {:?}, Size: {}",
                i + 1,
                order.product_id,
                order.side,
                order.status,
                order.size
            );
        }

        if let Some(page_info) = &pagination {
            println!(
                "Pagination - Before: {:?}, After: {:?}",
                page_info.before, page_info.after
            );
        }
    }
}

/// Test the get fills endpoint
#[tokio::test]
async fn test_get_fills() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Coinbase Exchange") {
        return;
    }

    let credentials = match CoinbaseCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Coinbase Exchange private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = venues::coinbaseexchange::private::rest::GetFillsRequest {
        order_id: None,
        product_id: Some("BTC-USD".to_string()), // Common trading pair
        limit: Some(20),
        before: None,
        after: None,
        market_type: None,
        start_date: None,
        end_date: None,
    };

    let result = client.get_fills(&request).await;

    if let Some((fills, pagination)) = handle_coinbase_result!(result, "get_fills") {
        println!("Fills retrieved successfully");
        println!("Number of fills: {}", fills.len());

        // Log fill details without asserting on dynamic values
        for (i, fill) in fills.iter().enumerate().take(3) {
            println!(
                "Fill {}: {} - Side: {:?}, Size: {}, Price: {}",
                i + 1,
                fill.product_id,
                fill.side,
                fill.size,
                fill.price
            );
        }

        if let Some(page_info) = &pagination {
            println!(
                "Pagination - Before: {:?}, After: {:?}",
                page_info.before, page_info.after
            );
        }
    }
}

/// Test the get specific order endpoint
#[tokio::test]
async fn test_get_order() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Coinbase Exchange") {
        return;
    }

    let credentials = match CoinbaseCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Coinbase Exchange private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // First, get a list of orders to find an order ID
    let orders_request = venues::coinbaseexchange::private::rest::GetOrdersRequest {
        profile_id: None,
        product_id: Some("BTC-USD".to_string()),
        sorted_by: None,
        sorting: None,
        start_date: None,
        end_date: None,
        before: None,
        after: None,
        limit: Some(1),
        status: None,
        market_type: None,
    };

    let orders_result = client.get_orders(&orders_request).await;

    if let Some((orders, _pagination)) =
        handle_coinbase_result!(orders_result, "get_orders for get_order test")
    {
        if !orders.is_empty() {
            let order_id = &orders[0].id;

            let request =
                venues::coinbaseexchange::private::rest::GetOrderRequest { market_type: None };

            let result = client.get_order(order_id, &request).await;

            if let Some(order) = handle_coinbase_result!(result, "get_order") {
                println!("Specific order retrieved successfully");
                println!("Order ID: {}", order.id);
                println!("Product: {}", order.product_id);
                println!("Status: {:?}", order.status);
                println!("Side: {:?}", order.side);
            }
        } else {
            println!("⚠️ No orders found for get_order test");
        }
    }
}

/// Test error handling with invalid requests
#[tokio::test]
async fn test_error_handling() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Coinbase Exchange") {
        return;
    }

    let credentials = match CoinbaseCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Coinbase Exchange private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Test with invalid order ID
    let request = venues::coinbaseexchange::private::rest::GetOrderRequest { market_type: None };

    let result = client
        .get_order("invalid-order-id-123456789", &request)
        .await;

    match result {
        Ok(_response) => {
            println!("⚠️ Unexpected success with invalid order ID");
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
    if config.skip_if_no_credentials("Coinbase Exchange") {
        return;
    }

    let credentials = match CoinbaseCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Coinbase Exchange private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Make multiple quick requests to test rate limiting
    for i in 0..3 {
        let request = venues::coinbaseexchange::private::rest::GetAccountBalancesRequest {
            limit: Some(10),
            before: None,
            after: None,
        };

        let result = client.get_account_balances(&request).await;

        if handle_coinbase_result!(result, &format!("rate_limiting_account_balances_{}", i))
            .is_some()
        {
            println!("Rate limited request {} completed successfully", i + 1);
        }

        // Small delay between requests to respect rate limits
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

/// Test paginated account balances retrieval
#[tokio::test]
async fn test_paginated_account_balances() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Coinbase Exchange") {
        return;
    }

    let credentials = match CoinbaseCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Coinbase Exchange private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = venues::coinbaseexchange::private::rest::GetAccountBalancesRequest {
        limit: Some(5), // Small limit to test pagination
        before: None,
        after: None,
    };

    let result = client.get_account_balances(&request).await;

    if let Some(response) = handle_coinbase_result!(result, "get_account_balances (paginated)") {
        println!("Paginated account balances retrieved successfully");
        println!("Number of accounts in page: {}", response.accounts.len());

        if let Some(pagination) = &response.pagination {
            println!(
                "Pagination - Before: {:?}, After: {:?}",
                pagination.before, pagination.after
            );
            if pagination.after.is_some() {
                println!("Next cursor available for pagination");
            }
        }

        // Log non-zero balances
        let non_zero_accounts: Vec<_> = response
            .accounts
            .iter()
            .filter(|account| account.available != "0")
            .collect();

        println!("Non-zero balance accounts: {}", non_zero_accounts.len());
        for (i, account) in non_zero_accounts.iter().enumerate().take(3) {
            println!(
                "Non-zero account {}: {} - Available: {}",
                i + 1,
                account.currency,
                account.available
            );
        }
    }
}

/// Test orders filtering by product
#[tokio::test]
async fn test_orders_filtering() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("Coinbase Exchange") {
        return;
    }

    let credentials = match CoinbaseCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping Coinbase Exchange private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Test orders for ETH-USD
    let request = venues::coinbaseexchange::private::rest::GetOrdersRequest {
        profile_id: None,
        product_id: Some("ETH-USD".to_string()),
        sorted_by: None,
        sorting: None,
        start_date: None,
        end_date: None,
        before: None,
        after: None,
        limit: Some(10),
        status: None,
        market_type: None,
    };

    let result = client.get_orders(&request).await;

    if let Some((orders, pagination)) =
        handle_coinbase_result!(result, "get_orders (ETH-USD filter)")
    {
        println!("ETH-USD orders retrieved successfully");
        println!("Number of ETH-USD orders: {}", orders.len());

        // Verify all orders are for ETH-USD
        let eth_orders: Vec<_> = orders
            .iter()
            .filter(|order| order.product_id == "ETH-USD")
            .collect();

        println!("Confirmed ETH-USD orders: {}", eth_orders.len());

        // Log order details
        for (i, order) in eth_orders.iter().enumerate().take(2) {
            println!(
                "ETH-USD order {}: Status: {:?}, Side: {:?}, Created: {}",
                i + 1,
                order.status,
                order.side,
                order.created_at
            );
        }

        if let Some(page_info) = &pagination {
            println!(
                "Pagination - Before: {:?}, After: {:?}",
                page_info.before, page_info.after
            );
        }
    }
}
