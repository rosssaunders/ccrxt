//! KuCoin Margin Debit API Example
//!
//! This example demonstrates how to use the KuCoin margin debit endpoints
//! for borrowing, repaying, checking interest, and modifying leverage.
//!
//! Required setup:
//! - Set environment variables:
//!   - KUCOIN_API_KEY: Your KuCoin API key
//!   - KUCOIN_API_SECRET: Your KuCoin API secret
//!   - KUCOIN_PASSPHRASE: Your KuCoin API passphrase
//!
//! Note: This example uses real API endpoints. Ensure you have the necessary
//! credentials and permissions for margin trading on your KuCoin account.

use std::env;

use rest::secrets::SecretString;
use venues::kucoin::private::rest::{
    BorrowRequest, GetBorrowHistoryRequest, GetInterestHistoryRequest, GetRepayHistoryRequest,
    ModifyLeverageRequest, RepayRequest, RestClient, TimeInForce,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with credentials from environment variables
    let api_key = env::var("KUCOIN_API_KEY").expect("KUCOIN_API_KEY not set");
    let api_secret = env::var("KUCOIN_API_SECRET").expect("KUCOIN_API_SECRET not set");
    let passphrase = env::var("KUCOIN_PASSPHRASE").expect("KUCOIN_PASSPHRASE not set");

    // Create REST client for private API using SecretString and boxing
    let client = RestClient::new_with_credentials(
        Box::new(SecretString::new(api_key.into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(api_secret.into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(passphrase.into())) as Box<dyn rest::secrets::ExposableSecret>,
    );

    println!("🏪 KuCoin Margin Debit API Example");
    println!("====================================\n");

    // 1. Borrow margin (simulation)
    println!("💸 Borrow margin example (simulation)");
    let borrow_request = BorrowRequest {
        currency: "USDT".to_string(),
        size: "10".to_string(),
        time_in_force: TimeInForce::ImmediateOrCancel,
        symbol: None, // Set Some("BTC-USDT".to_string()) for isolated
        is_isolated: Some(false),
        is_hf: Some(false),
    };
    println!("   Currency: {}", borrow_request.currency);
    println!(
        "   Amount: {} {}",
        borrow_request.size, borrow_request.currency
    );
    println!("   Time in force: {:?}", borrow_request.time_in_force);
    // Uncomment to actually borrow (requires sufficient margin)
    /*
    match client.borrow(borrow_request).await {
        Ok((response, _headers)) => {
            println!("✅ Borrow order placed! Order ID: {} | Actual size: {}", response.order_no, response.actual_size);
        }
        Err(e) => println!("❌ Failed to borrow: {}", e),
    }
    */

    // 2. Get borrow history
    println!("\n📋 Getting borrow history...");
    let borrow_history_request = GetBorrowHistoryRequest {
        currency: "USDT".to_string(),
        is_isolated: Some(false),
        symbol: None,
        order_no: None,
        start_time: None,
        end_time: None,
        current_page: Some(1),
        page_size: Some(5),
    };
    match client.get_borrow_history(borrow_history_request).await {
        Ok((response, _headers)) => {
            println!("✅ Borrow history found: {} items", response.total_num);
            for (i, item) in response.items.iter().take(3).enumerate() {
                println!(
                    "   {}. Order {}: {} {} (Status: {:?})",
                    i + 1,
                    item.order_no,
                    item.size,
                    item.currency,
                    item.status
                );
            }
        }
        Err(e) => println!("❌ Failed to get borrow history: {}", e),
    }

    // 3. Repay margin (simulation)
    println!("\n💵 Repay margin example (simulation)");
    let repay_request = RepayRequest {
        currency: "USDT".to_string(),
        size: "5".to_string(),
        symbol: None,
        is_isolated: Some(false),
        is_hf: Some(false),
    };
    println!("   Currency: {}", repay_request.currency);
    println!(
        "   Amount: {} {}",
        repay_request.size, repay_request.currency
    );
    // Uncomment to actually repay (requires valid borrow)
    /*
    match client.repay(repay_request).await {
        Ok((response, _headers)) => {
            println!("✅ Repay order placed! Order ID: {} | Actual size: {}", response.order_no, response.actual_size);
        }
        Err(e) => println!("❌ Failed to repay: {}", e),
    }
    */

    // 4. Get repay history
    println!("\n📋 Getting repay history...");
    let repay_history_request = GetRepayHistoryRequest {
        currency: "USDT".to_string(),
        is_isolated: Some(false),
        symbol: None,
        order_no: None,
        start_time: None,
        end_time: None,
        current_page: Some(1),
        page_size: Some(5),
    };
    match client.get_repay_history(repay_history_request).await {
        Ok((response, _headers)) => {
            println!("✅ Repay history found: {} items", response.total_num);
            for (i, item) in response.items.iter().take(3).enumerate() {
                println!(
                    "   {}. Order {}: {} {} principal, {} interest (Status: {:?})",
                    i + 1,
                    item.order_no,
                    item.principal,
                    item.currency,
                    item.interest,
                    item.status
                );
            }
        }
        Err(e) => println!("❌ Failed to get repay history: {}", e),
    }

    // 5. Get interest history
    println!("\n💡 Getting interest history...");
    let interest_history_request = GetInterestHistoryRequest {
        currency: Some("USDT".to_string()),
        is_isolated: Some(false),
        symbol: None,
        start_time: None,
        end_time: None,
        current_page: Some(1),
        page_size: Some(5),
    };
    match client.get_interest_history(interest_history_request).await {
        Ok((response, _headers)) => {
            println!("✅ Interest history found: {} items", response.total_num);
            for (i, item) in response.items.iter().take(3).enumerate() {
                println!(
                    "   {}. {}: {} interest at {}%",
                    i + 1,
                    item.currency,
                    item.interest_amount,
                    item.day_ratio
                );
            }
        }
        Err(e) => println!("❌ Failed to get interest history: {}", e),
    }

    // 6. Modify leverage (simulation)
    println!("\n⚙️  Modify leverage example (simulation)");
    let modify_leverage_request = ModifyLeverageRequest {
        symbol: Some("BTC-USDT".to_string()),
        is_isolated: Some(true),
        leverage: "5.0".to_string(),
    };
    println!("   Symbol: {:?}", modify_leverage_request.symbol);
    println!("   Leverage: {}", modify_leverage_request.leverage);
    // Uncomment to actually modify leverage
    /*
    match client.modify_leverage(modify_leverage_request).await {
        Ok((result, _headers)) => {
            println!("✅ Leverage modified: {}", result);
        }
        Err(e) => println!("❌ Failed to modify leverage: {}", e),
    }
    */

    println!("\n🎉 Margin Debit API example completed!");
    println!("\nKey concepts demonstrated:");
    println!("• 💸 Borrowing margin");
    println!("• 📋 Checking borrow and repay history");
    println!("• 💵 Repaying margin");
    println!("• 💡 Checking interest history");
    println!("• ⚙️  Modifying leverage");
    println!("\n⚠️  Remember: Margin trading involves risks!\n   Always check market conditions and your account status before borrowing.");

    Ok(())
}
