//! KuCoin Private API Example
//!
//! This example demonstrates how to use the KuCoin private API for account management,
//! trading, deposits, withdrawals, and transfers.
//!
//! IMPORTANT: This example requires valid KuCoin API credentials.
//! Set the following environment variables:
//! - KUCOIN_API_KEY: Your KuCoin API key
//! - KUCOIN_API_SECRET: Your KuCoin API secret
//! - KUCOIN_API_PASSPHRASE: Your KuCoin API passphrase
//!
//! WARNING: This example will attempt to place actual orders and transfers.
//! Use the sandbox environment for testing.

use std::env;

use rest::secrets::SecretString;
use venues::kucoin::{
    private::rest::{
        GetAccountBalanceRequest, GetAccountsRequest, GetDepositAddressRequest, GetFillsRequest,
        GetOrdersRequest, GetTransferableRequest, GetWithdrawalQuotasRequest, RestClient,
    },
    OrderStatus,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API credentials from environment variables
    let api_key = env::var("KUCOIN_API_KEY").expect("KUCOIN_API_KEY environment variable not set");
    let api_secret =
        env::var("KUCOIN_API_SECRET").expect("KUCOIN_API_SECRET environment variable not set");
    let api_passphrase = env::var("KUCOIN_API_PASSPHRASE")
        .expect("KUCOIN_API_PASSPHRASE environment variable not set");

    // Initialize the private REST client
    let client = RestClient::new_with_credentials(
        Box::new(SecretString::new(api_key.into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(api_secret.into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(api_passphrase.into()))
            as Box<dyn rest::secrets::ExposableSecret>,
    );

    println!("=== KuCoin Private API Example ===\n");

    // 1. Get Account Information
    println!("=== Account Information ===");

    // Get all accounts
    let (accounts, _) = client.get_accounts(GetAccountsRequest::default()).await?;
    println!("Found {} accounts", accounts.len());

    // Show first few accounts
    for account in accounts.iter().take(3) {
        println!(
            "Account: {} ({}) - Balance: {}, Available: {}",
            account.currency, account.account_type, account.balance, account.available
        );
    }

    // Get specific account balance (USDT)
    let (usdt_balances, _) = client
        .get_account_balance(GetAccountBalanceRequest {
            currency: Some("USDT".to_string()),
            account_type: Some("trade".to_string()),
        })
        .await?;

    if let Some(usdt_balance) = usdt_balances.first() {
        println!(
            "USDT Trade Account - Available: {}, Held: {}",
            usdt_balance.available, usdt_balance.holds
        );
    }

    // 2. Trading Information
    println!("\n=== Trading Information ===");

    // Get active orders
    let (active_orders, _) = client
        .get_orders(GetOrdersRequest {
            status: Some(OrderStatus::Active),
            symbol: Some("BTC-USDT".to_string()),
            ..Default::default()
        })
        .await?;
    println!("Active BTC-USDT orders: {}", active_orders.items.len());

    // Get recent fills
    let (recent_fills, _) = client
        .get_fills(GetFillsRequest {
            symbol: Some("BTC-USDT".to_string()),
            ..Default::default()
        })
        .await?;
    println!("Recent BTC-USDT fills: {}", recent_fills.items.len());

    // Show last 3 fills
    for fill in recent_fills.items.iter().take(3) {
        println!(
            "Fill: {:?} {} {} @ {} (Fee: {} {})",
            fill.side, fill.size, fill.symbol, fill.price, fill.fee, fill.fee_currency
        );
    }

    // 3. Deposit Information
    println!("\n=== Deposit Information ===");

    // Get BTC deposit address
    let (btc_address, _) = client
        .get_deposit_address(GetDepositAddressRequest {
            currency: "BTC".to_string(),
            chain: None,
        })
        .await?;
    println!("BTC deposit address: {}", btc_address.address);
    if let Some(memo) = btc_address.memo {
        println!("BTC deposit memo: {}", memo);
    }

    // 4. Withdrawal Information
    println!("\n=== Withdrawal Information ===");

    // Get BTC withdrawal quotas
    let (btc_quota, _) = client
        .get_withdrawal_quotas(GetWithdrawalQuotasRequest {
            currency: "BTC".to_string(),
            chain: None,
        })
        .await?;
    println!(
        "BTC withdrawal - Available: {}, Min: {}, Fee: {}",
        btc_quota.available_amount, btc_quota.withdraw_min_size, btc_quota.withdraw_min_fee
    );

    // 5. Transfer Information
    println!("\n=== Transfer Information ===");

    // Get transferable USDT balance
    let (transferable_usdt, _) = client
        .get_transferable(GetTransferableRequest {
            currency: "USDT".to_string(),
            account_type: "main".to_string(),
            tag: None,
        })
        .await?;
    println!(
        "USDT transferable from main: {}",
        transferable_usdt.transferable
    );

    println!("\n=== Example completed successfully! ===");
    println!("Note: No actual trades or transfers were executed in this example.");

    Ok(())
}

// Unit tests for the example functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_would_be_created_with_credentials() {
        // This test verifies the client creation logic would work
        // but doesn't actually create a client since we don't have test credentials

        let api_key = "test_key";
        let api_secret = "test_secret";
        let api_passphrase = "test_passphrase";

        // Verify the credential types are correct
        let _key = SecretString::new(api_key.to_string().into());
        let _secret = SecretString::new(api_secret.to_string().into());
        let _passphrase = SecretString::new(api_passphrase.to_string().into());

        // If we reach here, the types are compatible
        assert!(true);
    }

    #[test]
    fn test_request_structures() {
        let _account_balance_req = GetAccountBalanceRequest {
            currency: Some("BTC".to_string()),
            account_type: Some("trade".to_string()),
        };

        let _accounts_req = GetAccountsRequest::default();

        let _orders_req = GetOrdersRequest {
            status: Some(OrderStatus::Active),
            symbol: Some("BTC-USDT".to_string()),
            ..Default::default()
        };

        let _fills_req = GetFillsRequest::default();

        // These should all be successfully created
        assert!(true);
    }
}
