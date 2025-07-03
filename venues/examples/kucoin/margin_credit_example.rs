//! KuCoin Margin Credit API Example
//!
//! This example demonstrates how to use the KuCoin margin credit endpoints
//! for lending and earning interest on cryptocurrency holdings.
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
    GetLoanMarketInterestRateRequest, GetLoanMarketRequest, GetPurchaseOrdersRequest,
    GetRedeemOrdersRequest, ModifyPurchaseRequest, PurchaseOrderStatus, PurchaseRequest,
    RedeemOrderStatus, RedeemRequest, RestClient,
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

    println!("🏪 KuCoin Margin Credit API Example");
    println!("=====================================\n");

    // 1. Get loan market information for all currencies
    println!("📊 Getting loan market information...");
    let loan_market_request = GetLoanMarketRequest { currency: None };
    
    match client.get_loan_market(loan_market_request).await {
        Ok((markets, _headers)) => {
            println!("✅ Available lending markets: {}", markets.len());
            
            // Show details for first few markets
            for (i, market) in markets.iter().take(3).enumerate() {
                if let Some(currency) = &market.currency {
                    println!("   {}. {} - Min rate: {:.4}%, Max rate: {:.4}%", 
                        i + 1,
                        currency,
                        market.min_interest_rate.as_ref().unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0) * 100.0,
                        market.max_interest_rate.as_ref().unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0) * 100.0
                    );
                }
            }
        }
        Err(e) => println!("❌ Failed to get loan market: {}", e),
    }

    // 2. Get specific market information for BTC
    println!("\n📈 Getting BTC loan market information...");
    let btc_market_request = GetLoanMarketRequest {
        currency: Some("BTC".to_string()),
    };
    
    match client.get_loan_market(btc_market_request).await {
        Ok((markets, _headers)) => {
            if let Some(btc_market) = markets.first() {
                println!("✅ BTC lending market:");
                println!("   Purchase enabled: {}", btc_market.purchase_enable.unwrap_or(false));
                println!("   Redeem enabled: {}", btc_market.redeem_enable.unwrap_or(false));
                println!("   Min purchase: {} BTC", btc_market.min_purchase_size.as_ref().unwrap_or(&"N/A".to_string()));
                println!("   Max purchase: {} BTC", btc_market.max_purchase_size.as_ref().unwrap_or(&"N/A".to_string()));
                println!("   Market rate: {:.4}%", 
                    btc_market.market_interest_rate.as_ref().unwrap_or(&"0".to_string()).parse::<f64>().unwrap_or(0.0) * 100.0
                );
            }
        }
        Err(e) => println!("❌ Failed to get BTC market: {}", e),
    }

    // 3. Get market interest rate history for BTC
    println!("\n📉 Getting BTC market interest rate history...");
    let rate_history_request = GetLoanMarketInterestRateRequest {
        currency: "BTC".to_string(),
    };
    
    match client.get_loan_market_interest_rate(rate_history_request).await {
        Ok((rates, _headers)) => {
            println!("✅ BTC rate history (last 7 days): {} data points", rates.len());
            
            // Show latest rates
            for (i, rate) in rates.iter().take(3).enumerate() {
                println!("   {}. {}: {:.4}%", 
                    i + 1,
                    rate.time,
                    rate.market_interest_rate.parse::<f64>().unwrap_or(0.0) * 100.0
                );
            }
        }
        Err(e) => println!("❌ Failed to get rate history: {}", e),
    }

    // 4. Example: Purchase/Lend credit (commented out to avoid real transactions)
    println!("\n💰 Purchase/Lending example (simulation)");
    println!("   This would place a lending order to earn interest:");
    
    let purchase_request = PurchaseRequest {
        currency: "USDT".to_string(),
        size: "100".to_string(),       // Lend 100 USDT
        interest_rate: "0.05".to_string(),  // At 5% annual rate
    };
    
    println!("   Currency: {}", purchase_request.currency);
    println!("   Amount: {} {}", purchase_request.size, purchase_request.currency);
    println!("   Interest rate: {}% annually", 
        purchase_request.interest_rate.parse::<f64>().unwrap_or(0.0) * 100.0);
    
    // Uncomment to actually place the order (requires sufficient balance)
    /*
    match client.purchase(purchase_request).await {
        Ok((response, _headers)) => {
            println!("✅ Purchase order placed! Order ID: {}", response.order_no);
            
            // 5. Example: Modify the purchase order
            let modify_request = ModifyPurchaseRequest {
                currency: "USDT".to_string(),
                purchase_order_no: response.order_no.clone(),
                interest_rate: "0.06".to_string(),  // Change to 6% annual rate
            };
            
            match client.modify_purchase(modify_request).await {
                Ok((result, _headers)) => {
                    println!("✅ Purchase order modified: {}", result);
                }
                Err(e) => println!("❌ Failed to modify order: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to place purchase order: {}", e),
    }
    */

    // 6. Get purchase orders history
    println!("\n📋 Getting purchase orders history...");
    let purchase_orders_request = GetPurchaseOrdersRequest {
        status: PurchaseOrderStatus::Done,
        currency: Some("USDT".to_string()),
        purchase_order_no: None,
        current_page: Some(1),
        page_size: Some(10),
    };
    
    match client.get_purchase_orders(purchase_orders_request).await {
        Ok((response, _headers)) => {
            println!("✅ Purchase orders found: {}", response.total_num);
            println!("   Page: {}/{}", response.current_page, response.total_page);
            
            for (i, order) in response.items.iter().take(3).enumerate() {
                println!("   {}. Order {}: {} {} at {:.4}% (Status: {})", 
                    i + 1,
                    order.purchase_order_no,
                    order.purchase_size,
                    order.currency,
                    order.interest_rate.parse::<f64>().unwrap_or(0.0) * 100.0,
                    order.status
                );
            }
        }
        Err(e) => println!("❌ Failed to get purchase orders: {}", e),
    }

    // 7. Example: Redeem a loan order (commented out to avoid real transactions)
    println!("\n🔄 Redemption example (simulation)");
    println!("   This would redeem a lending position early:");
    
    let redeem_request = RedeemRequest {
        currency: "USDT".to_string(),
        size: "50".to_string(),           // Redeem 50 USDT
        purchase_order_no: "example-order-id".to_string(),
    };
    
    println!("   Currency: {}", redeem_request.currency);
    println!("   Amount: {} {}", redeem_request.size, redeem_request.currency);
    println!("   Purchase order: {}", redeem_request.purchase_order_no);
    
    // Uncomment to actually redeem (requires valid purchase order)
    /*
    match client.redeem(redeem_request).await {
        Ok((response, _headers)) => {
            println!("✅ Redemption order placed! Order ID: {}", response.order_no);
        }
        Err(e) => println!("❌ Failed to place redemption order: {}", e),
    }
    */

    // 8. Get redeem orders history
    println!("\n📋 Getting redeem orders history...");
    let redeem_orders_request = GetRedeemOrdersRequest {
        status: RedeemOrderStatus::Done,
        currency: Some("USDT".to_string()),
        redeem_order_no: None,
        current_page: Some(1),
        page_size: Some(10),
    };
    
    match client.get_redeem_orders(redeem_orders_request).await {
        Ok((response, _headers)) => {
            println!("✅ Redeem orders found: {}", response.total_num);
            println!("   Page: {}/{}", response.current_page, response.total_page);
            
            for (i, order) in response.items.iter().take(3).enumerate() {
                println!("   {}. Redeem {}: {} {} -> {} {} (Status: {})", 
                    i + 1,
                    order.redeem_order_no,
                    order.redeem_size,
                    order.currency,
                    order.receipt_size,
                    order.currency,
                    order.status
                );
            }
        }
        Err(e) => println!("❌ Failed to get redeem orders: {}", e),
    }

    println!("\n🎉 Margin Credit API example completed!");
    println!("\nKey concepts demonstrated:");
    println!("• 📊 Checking available lending markets and rates");
    println!("• 📈 Monitoring interest rate history");
    println!("• 💰 Lending cryptocurrency to earn interest (purchase)");
    println!("• ⚙️  Modifying lending orders");
    println!("• 🔄 Early redemption of lending positions");
    println!("• 📋 Tracking lending and redemption history");
    println!("\n⚠️  Remember: Cryptocurrency lending involves risks!");
    println!("   Always check market conditions and terms before lending.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        // Test that we can create request objects without network calls
        let loan_market_request = GetLoanMarketRequest {
            currency: Some("BTC".to_string()),
        };
        assert_eq!(loan_market_request.currency, Some("BTC".to_string()));

        let purchase_request = PurchaseRequest {
            currency: "USDT".to_string(),
            size: "100".to_string(),
            interest_rate: "0.05".to_string(),
        };
        assert_eq!(purchase_request.currency, "USDT");
        assert_eq!(purchase_request.size, "100");
        assert_eq!(purchase_request.interest_rate, "0.05");

        let redeem_request = RedeemRequest {
            currency: "USDT".to_string(),
            size: "50".to_string(),
            purchase_order_no: "test-order".to_string(),
        };
        assert_eq!(redeem_request.currency, "USDT");
        assert_eq!(redeem_request.size, "50");
        assert_eq!(redeem_request.purchase_order_no, "test-order");
    }

    #[test]
    fn test_status_enums() {
        // Test that status enums work correctly
        assert_eq!(PurchaseOrderStatus::Done, PurchaseOrderStatus::Done);
        assert_eq!(PurchaseOrderStatus::Pending, PurchaseOrderStatus::Pending);
        assert_eq!(RedeemOrderStatus::Done, RedeemOrderStatus::Done);
        assert_eq!(RedeemOrderStatus::Pending, RedeemOrderStatus::Pending);
    }
}
