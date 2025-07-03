/// BingX Symbols API Example
///
/// This example demonstrates how to fetch and display trading symbol information
/// from the BingX public API. This includes symbol metadata like price/quantity
/// steps, trading limits, and current status.
use reqwest::Client;
use venues::bingx::{
    public::PublicRestClient, GetSymbolsRequest, RateLimiter, SymbolStatus,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client and rate limiter
    let http_client = Client::new();
    let rate_limiter = RateLimiter::new();

    // Create the public REST client
    let client = PublicRestClient::new(
        "https://open-api.bingx.com",
        http_client,
        rate_limiter,
    );

    println!("BingX Symbols Information");
    println!("========================");

    // Get all trading symbols
    let symbols_request = GetSymbolsRequest::new(chrono::Utc::now().timestamp_millis());
    
    match client.get_symbols(&symbols_request).await {
        Ok(response) => {
            println!("Total symbols found: {}", response.symbols.len());
            println!();

            // Display some statistics
            let online_symbols = response.symbols.iter()
                .filter(|s| s.status == SymbolStatus::Online)
                .count();
            let offline_symbols = response.symbols.iter()
                .filter(|s| s.status == SymbolStatus::Offline)
                .count();
            let pre_open_symbols = response.symbols.iter()
                .filter(|s| s.status == SymbolStatus::PreOpen)
                .count();
            let suspended_symbols = response.symbols.iter()
                .filter(|s| s.status == SymbolStatus::TradingSuspended)
                .count();

            println!("Symbol Status Summary:");
            println!("  Online: {}", online_symbols);
            println!("  Offline: {}", offline_symbols);
            println!("  Pre-open: {}", pre_open_symbols);
            println!("  Trading suspended: {}", suspended_symbols);
            println!();

            // Display details for first 10 symbols
            println!("Symbol Details (first 10):");
            println!("{:-<120}", "");
            println!(
                "{:<15} {:<8} {:<12} {:<12} {:<15} {:<15} {:<8} {:<8}",
                "Symbol", "Status", "Tick Size", "Step Size", "Min Notional", "Max Notional", "API Buy", "API Sell"
            );
            println!("{:-<120}", "");

            for symbol in response.symbols.iter().take(10) {
                let status_str = match symbol.status {
                    SymbolStatus::Online => "Online",
                    SymbolStatus::Offline => "Offline",
                    SymbolStatus::PreOpen => "PreOpen",
                    SymbolStatus::TradingSuspended => "Suspended",
                };

                println!(
                    "{:<15} {:<8} {:<12.8} {:<12.8} {:<15.2} {:<15.2} {:<8} {:<8}",
                    symbol.symbol,
                    status_str,
                    symbol.tick_size,
                    symbol.step_size,
                    symbol.min_notional,
                    symbol.max_notional,
                    if symbol.api_state_buy { "Yes" } else { "No" },
                    if symbol.api_state_sell { "Yes" } else { "No" }
                );
            }

            // Find specific popular symbols
            println!("\nPopular Trading Pairs:");
            let popular_symbols = ["BTC-USDT", "ETH-USDT", "BNB-USDT", "ADA-USDT", "SOL-USDT"];
            
            for symbol_name in &popular_symbols {
                if let Some(symbol) = response.symbols.iter().find(|s| s.symbol == *symbol_name) {
                    println!("  {}: Status = {:?}, Min Trade = {:.2}, API Trading = {}/{}", 
                        symbol.symbol,
                        symbol.status,
                        symbol.min_notional,
                        if symbol.api_state_buy { "Buy" } else { "NoBuy" },
                        if symbol.api_state_sell { "Sell" } else { "NoSell" }
                    );
                } else {
                    println!("  {}: Not found", symbol_name);
                }
            }

        }
        Err(e) => {
            println!("Error getting symbols: {:?}", e);
        }
    }

    // Example of getting a specific symbol
    println!("\nGetting specific symbol (BTC-USDT):");
    let specific_request = GetSymbolsRequest::for_symbol(
        "BTC-USDT".to_string(),
        chrono::Utc::now().timestamp_millis()
    );

    match client.get_symbols(&specific_request).await {
        Ok(response) => {
            if let Some(symbol) = response.symbols.first() {
                println!("  Symbol: {}", symbol.symbol);
                println!("  Status: {:?}", symbol.status);
                println!("  Price precision: {:.8}", symbol.tick_size);
                println!("  Quantity precision: {:.8}", symbol.step_size);
                println!("  Minimum trade amount: {:.2}", symbol.min_notional);
                println!("  Maximum trade amount: {:.2}", symbol.max_notional);
                println!("  API buy enabled: {}", symbol.api_state_buy);
                println!("  API sell enabled: {}", symbol.api_state_sell);
                println!("  Online since: {}", symbol.time_online);
            }
        }
        Err(e) => {
            println!("Error getting specific symbol: {:?}", e);
        }
    }

    Ok(())
}
