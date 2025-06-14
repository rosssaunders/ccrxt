use anyhow::Result;
use rest::secrets::SecretValue;
use secrecy::SecretString;
use tracing::info;
use venues::binance::portfolio::{PrivateRestClient, RateLimiter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    info!("Binance Portfolio Margin API Client Example");

    // Create HTTP client
    let http_client = reqwest::Client::new();

    // Create rate limiter
    let rate_limiter = RateLimiter::new();

    // Create Portfolio Margin private client with placeholder credentials
    let api_key = std::env::var("BINANCE_API_KEY").unwrap_or_else(|_| "your_api_key".to_string());
    let api_secret =
        std::env::var("BINANCE_API_SECRET").unwrap_or_else(|_| "your_api_secret".to_string());

    let _portfolio_margin_client = PrivateRestClient::new(
        Box::new(SecretValue::new(SecretString::from(api_key))),
        Box::new(SecretValue::new(SecretString::from(api_secret))),
        "https://papi.binance.com",
        rate_limiter,
        http_client,
    );

    info!("Portfolio Margin private client created successfully!");
    info!("Base URL: https://papi.binance.com");
    info!("Client is ready to make authenticated requests to Portfolio Margin API");

    // Note: To make actual API calls, you would need to implement specific endpoint methods
    // similar to the coinm implementation (e.g., account info, position risk, etc.)

    Ok(())
}
