///! KuCoin Public API Example
///!
///! This example demonstrates how to use the KuCoin public API to fetch
///! market data such as server time, symbols, and ticker information.
///!
///! This example does not require any credentials as it only uses public endpoints.

use venues::kucoin::public::rest::{
    GetAllSymbolsRequest, GetAllTickersRequest, GetServerTimeRequest, RestClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the public REST client
    let client = RestClient::new_default();

    // 1. Get server time
    println!("=== Getting Server Time ===");
    let (server_time_response, _) = client
        .get_server_time(GetServerTimeRequest::default())
        .await?;
    println!("Server time: {} ms", server_time_response.timestamp);

    // 2. Get all symbols
    println!("\n=== Getting All Symbols ===");
    let (symbols_response, _) = client
        .get_all_symbols(GetAllSymbolsRequest::default())
        .await?;
    println!("Found {} symbols", symbols_response.len());

    // Show first 5 symbols as examples
    for symbol in symbols_response.iter().take(5) {
        println!(
            "Symbol: {} ({}/{}), Trading: {}",
            symbol.symbol, symbol.base_currency, symbol.quote_currency, symbol.enable_trading
        );
    }

    // 3. Get all tickers
    println!("\n=== Getting All Tickers ===");
    let (tickers_response, _) = client
        .get_all_tickers(GetAllTickersRequest::default())
        .await?;
    println!("Found {} tickers", tickers_response.ticker.len());

    // Show first 5 tickers as examples
    for ticker in tickers_response.ticker.iter().take(5) {
        let name = ticker.name.as_deref().unwrap_or("N/A");
        let last_price = ticker.last_price.as_deref().unwrap_or("N/A");
        let change_percentage = ticker.change_percentage.as_deref().unwrap_or("N/A");
        let change_price = ticker.change_price.as_deref().unwrap_or("N/A");
        println!(
            "Ticker: {} ({}) - Last: {}, 24h Change: {}% ({})",
            ticker.symbol, name, last_price, change_percentage, change_price
        );
    }

    println!("\n=== Example completed successfully! ===");
    Ok(())
}

// Unit tests for the example functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = RestClient::new_default();
        assert_eq!(client.base_url, "https://api.kucoin.com");
    }

    #[test]
    fn test_request_structures() {
        let _server_time_req = GetServerTimeRequest::default();
        let symbols_req = GetAllSymbolsRequest::default();
        let _tickers_req = GetAllTickersRequest::default();

        // These should all be successfully created
        assert!(symbols_req.market.is_none());
    }
}
