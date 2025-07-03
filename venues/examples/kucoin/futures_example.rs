//! KuCoin Futures REST API Example
//!
//! This example demonstrates how to use the KuCoin futures REST API to:
//! - Get contract information
//! - Get current funding rates
//! - Place and manage orders
//! - Get position information
//! 
//! To run this example, you need to set the following environment variables:
//! - KUCOIN_API_KEY: Your KuCoin API key
//! - KUCOIN_API_SECRET: Your KuCoin API secret
//! - KUCOIN_API_PASSPHRASE: Your KuCoin API passphrase

use venues::kucoin;
use rest::secrets::SecretString;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create public futures client
    let public_client = kucoin::public::futures::RestClient::new_default();

    println!("=== KuCoin Futures Public API Example ===");

    // Get contract information for XBTUSDTM
    println!("\n1. Getting contract information for XBTUSDTM...");
    let contract_request = kucoin::public::futures::GetContractRequest {
        symbol: "XBTUSDTM".to_string(),
    };

    match public_client.get_contract(contract_request).await {
        Ok((response, _headers)) => {
            let contract = response.data;
            println!("Contract: {}", contract.symbol);
            println!("Base Currency: {}", contract.base_currency);
            println!("Quote Currency: {}", contract.quote_currency);
            println!("Lot Size: {}", contract.lot_size);
            println!("Tick Size: {}", contract.tick_size);
            println!("Maker Fee Rate: {}", contract.maker_fee_rate);
            println!("Taker Fee Rate: {}", contract.taker_fee_rate);
            println!("Status: {:?}", contract.status);
        }
        Err(e) => println!("Error getting contract: {}", e),
    }

    // Get current funding rate
    println!("\n2. Getting current funding rate for XBTUSDTM...");
    let funding_request = kucoin::public::futures::GetCurrentFundingRateRequest {
        symbol: "XBTUSDTM".to_string(),
    };

    match public_client.get_current_funding_rate(funding_request).await {
        Ok((response, _headers)) => {
            let funding_rate = response.data;
            println!("Symbol: {}", funding_rate.symbol);
            println!("Current Funding Rate: {}", funding_rate.value);
            println!("Predicted Funding Rate: {}", funding_rate.predicted_value);
            println!("Time Point: {}", funding_rate.time_point);
        }
        Err(e) => println!("Error getting funding rate: {}", e),
    }

    // Get all active contracts
    println!("\n3. Getting all active contracts...");
    let all_contracts_request = kucoin::public::futures::GetAllContractsRequest;

    match public_client.get_all_contracts(all_contracts_request).await {
        Ok((response, _headers)) => {
            let contracts = response.data;
            println!("Found {} active contracts", contracts.len());
            for contract in contracts.iter().take(5) {
                println!("  - {}: {} ({:?})", contract.symbol, contract.settle_currency, contract.status);
            }
            if contracts.len() > 5 {
                println!("  ... and {} more", contracts.len() - 5);
            }
        }
        Err(e) => println!("Error getting all contracts: {}", e),
    }

    // Try private API if credentials are available
    if let (Ok(api_key), Ok(api_secret), Ok(api_passphrase)) = (
        env::var("KUCOIN_API_KEY"),
        env::var("KUCOIN_API_SECRET"),
        env::var("KUCOIN_API_PASSPHRASE"),
    ) {
        println!("\n=== KuCoin Futures Private API Example ===");

        // Create private futures client
        let private_client = kucoin::private::futures::RestClient::new_with_credentials(
            Box::new(SecretString::new(api_key.into())) as Box<dyn rest::secrets::ExposableSecret>,
            Box::new(SecretString::new(api_secret.into())) as Box<dyn rest::secrets::ExposableSecret>,
            Box::new(SecretString::new(api_passphrase.into())) as Box<dyn rest::secrets::ExposableSecret>,
        );

        // Get all positions
        println!("\n4. Getting all positions...");
        let positions_request = kucoin::private::futures::GetAllPositionsRequest;

        match private_client.get_all_positions(positions_request).await {
            Ok((response, _headers)) => {
                let positions = response.data;
                println!("Found {} positions", positions.len());
                for position in &positions {
                    println!("  - {}: {:?} {} (PnL: {})",
                        position.symbol,
                        position.side,
                        position.open_size,
                        position.unrealized_pnl
                    );
                }
            }
            Err(e) => println!("Error getting positions: {}", e),
        }

        // Get margin mode for XBTUSDTM
        println!("\n5. Getting margin mode for XBTUSDTM...");
        let margin_mode_request = kucoin::private::futures::GetMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
        };

        match private_client.get_margin_mode(margin_mode_request).await {
            Ok((response, _headers)) => {
                let margin_mode = response.data;
                println!("Symbol: {}", margin_mode.symbol);
                println!("Margin Mode: {:?}", margin_mode.margin_mode);
                println!("Cross Margin Leverage: {}", margin_mode.cross_margin_leverage);
                println!("Isolated Margin Leverage: {}", margin_mode.isolated_margin_leverage);
            }
            Err(e) => println!("Error getting margin mode: {}", e),
        }

        // Get orders (recent orders)
        println!("\n6. Getting recent orders...");
        let orders_request = kucoin::private::futures::GetOrdersRequest {
            status: Some(kucoin::OrderStatus::Done),
            symbol: None,
            side: None,
            order_type: None,
            start_at: None,
            end_at: None,
            current_page: Some(1),
            page_size: Some(10),
        };

        match private_client.get_orders(orders_request).await {
            Ok((response, _headers)) => {
                let orders = response.data;
                println!("Found {} orders (page {} of {})",
                    orders.items.len(),
                    orders.current_page,
                    orders.total_page
                );
                for order in &orders.items {
                    println!("  - {}: {:?} {:?} {} @ {} (Status: {:?})",
                        order.symbol,
                        order.side,
                        order.order_type,
                        order.size,
                        order.price.as_ref().unwrap_or(&"market".to_string()),
                        order.status
                    );
                }
            }
            Err(e) => println!("Error getting orders: {}", e),
        }

        println!("\n=== Example completed successfully! ===");
    } else {
        println!("\n=== Private API Example Skipped ===");
        println!("To test private API endpoints, set these environment variables:");
        println!("  KUCOIN_API_KEY=your_api_key");
        println!("  KUCOIN_API_SECRET=your_api_secret");
        println!("  KUCOIN_API_PASSPHRASE=your_api_passphrase");
    }

    Ok(())
}
