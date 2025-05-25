// Example: Place a trade on Binance COIN-M using the coinm Rust module
// Loads API credentials from .env
use std::{env};
use dotenv::dotenv;
use venues::binance::coinm::{BinanceCoinMPrivateRest, BinanceCoinMError, BinanceCoinMAPIError};
use venues::binance::coinm::{OrderRequest, OrderSide, OrderType, TimeInForce};
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env");
    let api_secret = env::var("API_SECRET").expect("API_SECRET not set in .env");
    let mut base_url = "https://dapi.binance.com".to_string();
    base_url = "https://testnet.binancefuture.com".to_string();

    // Create the client
    let client = BinanceCoinMPrivateRest::new(
        api_key,
        api_secret,
        base_url
    );

    // Fetch account information
    let result = client.get_account().await;

    match result {
        Ok(account) => {
            println!("Account fetched successfully");
            
            // Print all assets in a nice table
            if !account.data.assets.is_empty() {
                println!("{:<15} {:<20} {:<20} {:<20}", "Asset", "Wallet Balance", "Unrealized PNL", "Margin Balance");
                println!("{}", "-".repeat(75));
                
                for asset in &account.data.assets {
                    println!("{:<15} {:<20} {:<20} {:<20}", 
                        asset.asset,
                        asset.wallet_balance,
                        asset.unrealized_profit,
                        asset.margin_balance
                    );
                }
            } else {
                println!("No assets found in account");
            }

            // Example: Place a market order
            // Note: Uncomment to place an actual order - will use real funds on mainnet!
            /*
            let market_order = OrderRequest {
                symbol: "BTCUSD_PERP".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Market,
                quantity: Some(Decimal::from(1)), // 1 contract
                positionSide: None, // Default BOTH for One-way Mode
                timeInForce: None, // Not required for MARKET orders
                reduceOnly: Some("false".to_string()),
                price: None, // Not required for MARKET orders
                newClientOrderId: Some("my_order_123".to_string()),
                stopPrice: None,
                closePosition: None,
                activationPrice: None,
                callbackRate: None,
                workingType: None,
                priceProtect: None,
                newOrderRespType: None,
                priceMatch: None,
                selfTradePreventionMode: None,
                recvWindow: None,
                timestamp: None,
            };

            match client.place_order(market_order).await {
                Ok(order_result) => {
                    println!("Order placed successfully!");
                    println!("Order ID: {}", order_result.data.orderId);
                    println!("Symbol: {}", order_result.data.symbol);
                    println!("Status: {:?}", order_result.data.status);
                },
                Err(err) => handle_error(err),
            }

            // Example: Place a limit order
            let limit_order = OrderRequest {
                symbol: "BTCUSD_PERP".to_string(),
                side: OrderSide::Sell,
                order_type: OrderType::Limit,
                quantity: Some(Decimal::from(1)),
                positionSide: None,
                timeInForce: Some(TimeInForce::GTC), // Good Till Cancelled
                reduceOnly: None,
                price: Some(Decimal::from(50000)), // Sell at $50,000
                newClientOrderId: None,
                stopPrice: None,
                closePosition: None,
                activationPrice: None,
                callbackRate: None,
                workingType: None,
                priceProtect: None,
                newOrderRespType: None,
                priceMatch: None,
                selfTradePreventionMode: None,
                recvWindow: None,
                timestamp: None,
            };

            match client.place_order(limit_order).await {
                Ok(order_result) => {
                    println!("Limit order placed successfully!");
                    println!("Order ID: {}", order_result.data.orderId);
                    println!("Symbol: {}", order_result.data.symbol);
                    println!("Status: {:?}", order_result.data.status);
                },
                Err(err) => handle_error(err),
            }
            */
        },
        Err(err) => handle_error(err),
    }

    Ok(())
}

fn handle_error(err: BinanceCoinMError) {
    match err {
        BinanceCoinMError::ApiError(err) => {
            // Convert to typed error for better handling
            let typed_error: BinanceCoinMAPIError = err;
            
            // Handle all error types with detailed messages
            match &typed_error {
                // Authentication errors
                BinanceCoinMAPIError::RejectedMbxKey { .. }
                | BinanceCoinMAPIError::Unauthorized { .. }
                | BinanceCoinMAPIError::InvalidSignature { .. }
                | BinanceCoinMAPIError::BadApiKeyFmt { .. } => {
                    eprintln!("🔐 Authentication Error: {}", typed_error);
                    eprintln!("💡 Check your API credentials and permissions");
                },
                // Rate limiting errors
                BinanceCoinMAPIError::TooManyRequests { .. } => {
                    eprintln!("⏱️  Rate Limited: Too many requests");
                },
                BinanceCoinMAPIError::IpBanned { .. } => {
                    eprintln!("🚫 IP Banned: IP banned until reset");
                    eprintln!("💡 Solution: Use WebSocket for live updates to avoid IP bans");
                },
                // Connection/Network errors
                BinanceCoinMAPIError::Disconnected { .. } => {
                    eprintln!("📡 Connection Lost: Server temporarily unavailable");
                },
                BinanceCoinMAPIError::Timeout { .. } => {
                    eprintln!("⏰ Request Timeout: Server took too long to respond");
                },
                // Trading-specific errors
                BinanceCoinMAPIError::BalanceNotSufficient { .. } => {
                    eprintln!("💰 Insufficient Balance: Not enough funds for this operation");
                },
                BinanceCoinMAPIError::MarginNotSufficient { .. } => {
                    eprintln!("📈 Insufficient Margin: Not enough margin for this position");
                },
                BinanceCoinMAPIError::UserInLiquidation { .. } => {
                    eprintln!("⚠️  Account in Liquidation: Cannot place orders while in liquidation");
                },
                BinanceCoinMAPIError::MaxOpenOrderExceeded { .. } => {
                    eprintln!("📊 Too Many Open Orders: Reached maximum open order limit");
                },
                // Validation errors
                BinanceCoinMAPIError::InvalidTimestamp { .. } => {
                    eprintln!("🕐 Invalid Timestamp");
                },
                BinanceCoinMAPIError::BadSymbol { .. } => {
                    eprintln!("🎯 Invalid Symbol: The trading symbol is not valid");
                },
                BinanceCoinMAPIError::InvalidLeverage { .. } => {
                    eprintln!("⚙️  Invalid Leverage");
                },
                // Generic fallback for other API errors
                _ => {
                    eprintln!("🔴 API Error: {}", typed_error);
                }
            }
        },
        BinanceCoinMError::HttpError(err) => {
            eprintln!("HTTP/Network error: {}", err);
        },
        BinanceCoinMError::Error(msg) => {
            eprintln!("General error: {}", msg);
        },
    }
}
