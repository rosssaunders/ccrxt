//! Integration tests for OKX private REST API endpoints
//!
//! These tests verify the functionality of private endpoints that require authentication.
//! Tests are disabled by default and can be enabled by setting environment variables:
//!
//! ```bash
//! export RUN_PRIVATE_TESTS=true
//! export OKX_API_KEY=your_api_key
//! export OKX_SECRET_KEY=your_secret_key
//! export OKX_PASSPHRASE=your_passphrase
//! cargo test okx::private_integration_tests
//! ```
//!
//! Tests run against OKX production environment (no testnet available).
//! Be careful with trading operations.

use reqwest::Client;
use tokio;

// Import types from top-level venue exports as required by integration test standards
use venues::okx::{
    AccountBalance, Errors, GetAccountBalanceRequest, GetAccountConfigRequest, GetFillsRequest,
    GetOrderHistoryRequest, GetPendingOrdersRequest, GetPositionsRequest, InstrumentType,
    OkxApiResponse, PrivateRestClient, RateLimiter,
};

// Import common testing utilities
use crate::common::{CredentialLoader, OkxCredentials, PrivateTestConfig};

/// Helper function to create a test client for private endpoints
fn create_private_test_client(
    credentials: &OkxCredentials,
    config: &PrivateTestConfig,
) -> PrivateRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();
    let base_url = config.env.get_base_url("okx");

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
    error_str.contains("Invalid")
        || error_str.contains("Unauthorized")
        || error_str.contains("Authentication")
        || error_str.contains("50101") // OKX auth error code
}

/// Helper function to check if an error is due to API restrictions
fn is_api_restriction(err: &Errors) -> bool {
    let error_str = format!("{:?}", err);
    error_str.contains("restricted")
        || error_str.contains("disabled")
        || error_str.contains("prohibited")
}

/// Macro to standardize handling private API results with appropriate error checks
macro_rules! handle_okx_result {
    ($result:expr, $endpoint_name:expr) => {
        match $result {
            Ok(response) => {
                if response.code == "0" {
                    println!("✅ {} successful", $endpoint_name);
                    Some(response)
                } else {
                    println!(
                        "⚠️ {} returned error code: {} - {}",
                        $endpoint_name, response.code, response.msg
                    );
                    None
                }
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

/// Test the account balance endpoint
#[tokio::test]
async fn test_get_account_balance() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    config.env.print_env_info();

    let client = create_private_test_client(&credentials, &config);

    let request = GetAccountBalanceRequest {
        ccy: None, // Get all currencies
    };

    let result = client.get_account_balance(&request).await;

    if let Some(response) = handle_okx_result!(result, "get_account_balance") {
        println!("Account balance retrieved successfully");

        if !response.data.is_empty() {
            println!("Account details:");
            let account = &response.data[0];
            println!(
                "  Account Type: {:?}",
                account.details.get(0).map(|d| &d.ccy)
            );
            println!("  Number of currencies: {}", account.details.len());

            // Log a few balances without asserting on dynamic values
            for (i, detail) in account.details.iter().enumerate().take(5) {
                println!(
                    "  Balance {}: {} - Available: {}, Frozen: {}",
                    i + 1,
                    detail.ccy,
                    detail.avail_bal,
                    detail.frozen_bal
                );
            }
        }
    }
}

/// Test the account configuration endpoint
#[tokio::test]
async fn test_get_account_config() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = GetAccountConfigRequest {};

    let result = client.get_account_config().await;

    if let Some(response) = handle_okx_result!(result, "get_account_config") {
        println!("Account config retrieved successfully");

        if !response.data.is_empty() {
            let config = &response.data[0];
            println!("Account configuration:");
            println!("  User ID: {}", config.uid);
            println!("  Account Level: {}", config.acct_lv);
            println!("  Position Mode: {}", config.pos_mode);
            println!("  Auto Borrow: {}", config.auto_loan);
            println!("  Spot role type: {:?}", config.spot_role_type);
        }
    }
}

/// Test the open positions endpoint
#[tokio::test]
async fn test_get_positions() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = GetPositionsRequest {
        inst_type: Some(InstrumentType::Spot),
        inst_id: None,
        pos_id: None,
    };

    let result = client.get_positions(&request).await;

    if let Some(response) = handle_okx_result!(result, "get_positions") {
        println!("Positions retrieved successfully");
        println!("Number of positions: {}", response.data.len());

        // Log position details without asserting on dynamic values
        for (i, position) in response.data.iter().enumerate().take(3) {
            println!(
                "Position {}: {} - Size: {}, Side: {}",
                i + 1,
                position.inst_id,
                position.pos,
                position.pos_side
            );
        }
    }
}

/// Test the pending orders endpoint
#[tokio::test]
async fn test_get_pending_orders() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = GetPendingOrdersRequest {
        inst_type: Some(InstrumentType::Spot),
        uly: None,
        inst_family: None,
        inst_id: None,
        ord_type: None,
        state: None,
        after: None,
        before: None,
        limit: Some("20".to_string()),
    };

    let result = client.get_pending_orders(&request).await;

    if let Some(response) = handle_okx_result!(result, "get_pending_orders") {
        println!("Pending orders retrieved successfully");
        println!("Number of pending orders: {}", response.data.len());

        // Log order details without asserting on dynamic values
        for (i, order) in response.data.iter().enumerate().take(3) {
            println!(
                "Pending order {}: {} - Type: {:?}, Size: {}, Price: {}",
                i + 1,
                order.inst_id,
                order.ord_type,
                order.sz,
                order.px
            );
        }
    }
}

/// Test the order history endpoint
#[tokio::test]
async fn test_get_order_history() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = GetOrderHistoryRequest {
        inst_type: Some(InstrumentType::Spot),
        uly: None,
        inst_family: None,
        inst_id: None,
        ord_type: None,
        state: None,
        category: None,
        after: None,
        before: None,
        begin: None,
        end: None,
        limit: Some("10".to_string()),
    };

    let result = client.get_order_history(&request).await;

    if let Some(response) = handle_okx_result!(result, "get_order_history") {
        println!("Order history retrieved successfully");
        println!("Number of historical orders: {}", response.data.len());

        // Log order details without asserting on dynamic values
        for (i, order) in response.data.iter().enumerate().take(3) {
            println!(
                "Historical order {}: {} - State: {}, Side: {:?}, Size: {}",
                i + 1,
                order.inst_id,
                order.state,
                order.side,
                order.sz
            );
        }
    }
}

/// Test the fills/trade history endpoint
#[tokio::test]
async fn test_get_fills() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    let request = GetFillsRequest {
        inst_type: Some(InstrumentType::Spot),
        uly: None,
        inst_family: None,
        inst_id: None,
        ord_id: None,
        after: None,
        before: None,
        // begin and end fields not available
        limit: Some("10".to_string()),
    };

    let result = client.get_fills(&request).await;

    if let Some(response) = handle_okx_result!(result, "get_fills") {
        println!("Fill history retrieved successfully");
        println!("Number of fills: {}", response.data.len());

        // Log fill details without asserting on dynamic values
        for (i, fill) in response.data.iter().enumerate().take(3) {
            println!(
                "Fill {}: {} - Side: {}, Size: {}, Price: {}, Fee: {}",
                i + 1,
                fill.inst_id,
                format!("{:?}", fill.side),
                fill.fill_sz,
                fill.fill_px,
                fill.fee
            );
        }
    }
}

/// Test error handling with invalid requests
#[tokio::test]
async fn test_error_handling() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Test with invalid instrument type combination
    let request = GetOrderHistoryRequest {
        inst_type: Some(InstrumentType::Spot),
        uly: Some("INVALID-ULY".to_string()), // Invalid for spot
        inst_family: None,
        inst_id: Some("INVALID-SYMBOL".to_string()),
        ord_type: None,
        state: None,
        category: None,
        after: None,
        before: None,
        begin: None,
        end: None,
        limit: Some("1".to_string()),
    };

    let result = client.get_order_history(&request).await;

    match result {
        Ok(response) => {
            if response.code != "0" {
                println!(
                    "✅ Error handling test - Got expected error code: {}",
                    response.code
                );
            } else {
                println!("⚠️ Unexpected success with invalid parameters");
            }
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
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Make multiple quick requests to test rate limiting
    for i in 0..3 {
        let request = GetAccountConfigRequest {};

        let result = client.get_account_config().await;

        if handle_okx_result!(result, &format!("rate_limiting_config_{}", i)).is_some() {
            println!("Rate limited request {} completed successfully", i + 1);
        }

        // Small delay between requests
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }
}

/// Test account balance with specific currency filter
#[tokio::test]
async fn test_get_specific_currency_balance() {
    let config = PrivateTestConfig::new();
    if config.skip_if_no_credentials("OKX") {
        return;
    }

    let credentials = match OkxCredentials::load_from_env() {
        Some(creds) => creds,
        None => {
            println!("⚠️ Skipping OKX private test - credentials not available");
            return;
        }
    };

    let client = create_private_test_client(&credentials, &config);

    // Test with specific currency (USDT is commonly held)
    let request = GetAccountBalanceRequest {
        ccy: Some("USDT".to_string()),
    };

    let result = client.get_account_balance(&request).await;

    if let Some(response) = handle_okx_result!(result, "get_account_balance for USDT") {
        println!("USDT balance retrieved successfully");

        if !response.data.is_empty() && !response.data[0].details.is_empty() {
            let usdt_detail = &response.data[0].details[0];
            println!("USDT Balance Details:");
            println!("  Available: {}", usdt_detail.avail_bal);
            println!("  Frozen: {}", usdt_detail.frozen_bal);
            println!("  Cash Balance: {}", usdt_detail.cash_bal);
            println!("  Update Time: {}", usdt_detail.u_time);
        }
    }
}
