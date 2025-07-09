use reqwest::Client;
use venues::bitget::public::rest::{GetSymbolInfoRequest, GetTickerRequest, RestClient};
use venues::bitget::rate_limit::RateLimiter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Client::new();
    let rate_limiter = RateLimiter::default();
    let rest_client = RestClient::new("https://api.bitget.com", rate_limiter, client);

    // Test getting symbol info
    println!("Fetching symbol information...");
    let symbol_response = rest_client
        .get_symbol_info(GetSymbolInfoRequest::new().symbol("BTCUSDT"))
        .await?;

    println!("Symbol info response: {:#?}", symbol_response.data);

    // Test getting ticker
    println!("\nFetching ticker information...");
    let ticker_response = rest_client
        .get_ticker(GetTickerRequest::new().symbol("BTCUSDT"))
        .await?;

    println!("Ticker response: {:#?}", ticker_response.data);

    Ok(())
}
