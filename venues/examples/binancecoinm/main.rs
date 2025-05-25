// Example: Place a trade on Binance COIN-M using the coinm Rust module
// Loads API credentials from .env
use std::{env};
use dotenv::dotenv;
use venues::binance::coinm::{BinanceCoinMPrivateRest, BinanceCoinMError, BinanceCoinMAPIError};

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
        },
        Err(BinanceCoinMError::ApiError(err)) => {
            // Convert to typed error for better handling
            let typed_error: BinanceCoinMAPIError = err.into();
            
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
        Err(BinanceCoinMError::HttpError(err)) => {
            eprintln!("HTTP/Network error: {}", err);
        },
        Err(BinanceCoinMError::Error(msg)) => {
            eprintln!("General error: {}", msg);
        },
    }

    Ok(())
}
