/// Example of using OKX public API to get instruments
/// 
/// This example demonstrates how to retrieve instrument information from OKX exchange
/// using the /api/v5/public/instruments endpoint.
use venues::okx::{GetInstrumentsRequest, InstrumentType, PublicRestClient, RateLimiter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client and rate limiter
    let http_client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();
    
    // Create OKX public REST client
    let client = PublicRestClient::new("https://www.okx.com", http_client, rate_limiter);
    
    // Example 1: Get all SPOT instruments
    println!("=== Getting SPOT instruments ===");
    let spot_request = GetInstrumentsRequest {
        inst_type: InstrumentType::Spot,
        underlying: None,
        inst_family: None,
        inst_id: None,
    };
    
    match client.get_instruments(spot_request).await {
        Ok(response) => {
            println!("Response code: {}", response.code);
            println!("Found {} SPOT instruments", response.data.len());
            
            // Show first few instruments
            for (i, instrument) in response.data.iter().take(3).enumerate() {
                println!("  {}. {} - {} (State: {:?})", 
                    i + 1, 
                    instrument.inst_id, 
                    format!("{}/{}", instrument.base_ccy, instrument.quote_ccy),
                    instrument.state
                );
            }
        }
        Err(e) => eprintln!("Error getting SPOT instruments: {:?}", e),
    }
    
    // Example 2: Get specific instrument by ID
    println!("\n=== Getting specific instrument: BTC-USDT ===");
    let specific_request = GetInstrumentsRequest {
        inst_type: InstrumentType::Spot,
        underlying: None,
        inst_family: None,
        inst_id: Some("BTC-USDT".to_string()),
    };
    
    match client.get_instruments(specific_request).await {
        Ok(response) => {
            if let Some(instrument) = response.data.first() {
                println!("Instrument: {}", instrument.inst_id);
                println!("  Base currency: {}", instrument.base_ccy);
                println!("  Quote currency: {}", instrument.quote_ccy);
                println!("  Tick size: {}", instrument.tick_sz);
                println!("  Lot size: {}", instrument.lot_sz);
                println!("  Min size: {}", instrument.min_sz);
                println!("  State: {:?}", instrument.state);
            }
        }
        Err(e) => eprintln!("Error getting BTC-USDT instrument: {:?}", e),
    }
    
    // Example 3: Get perpetual swap instruments
    println!("\n=== Getting SWAP instruments ===");
    let swap_request = GetInstrumentsRequest {
        inst_type: InstrumentType::Swap,
        underlying: None,
        inst_family: None,
        inst_id: None,
    };
    
    match client.get_instruments(swap_request).await {
        Ok(response) => {
            println!("Found {} SWAP instruments", response.data.len());
            
            // Show BTC swaps
            let btc_swaps: Vec<_> = response.data.iter()
                .filter(|i| i.inst_id.contains("BTC"))
                .take(3)
                .collect();
                
            for instrument in btc_swaps {
                println!("  {} - Settlement: {} (Contract: {})", 
                    instrument.inst_id,
                    instrument.settle_ccy,
                    instrument.ct_type.as_ref().unwrap_or(&"N/A".to_string())
                );
            }
        }
        Err(e) => eprintln!("Error getting SWAP instruments: {:?}", e),
    }
    
    // Example 4: Get futures with specific underlying
    println!("\n=== Getting FUTURES instruments for BTC-USD ===");
    let futures_request = GetInstrumentsRequest {
        inst_type: InstrumentType::Futures,
        underlying: Some("BTC-USD".to_string()),
        inst_family: None,
        inst_id: None,
    };
    
    match client.get_instruments(futures_request).await {
        Ok(response) => {
            println!("Found {} BTC-USD FUTURES", response.data.len());
            
            for instrument in response.data.iter().take(3) {
                println!("  {} - Expiry: {} (Alias: {})", 
                    instrument.inst_id,
                    instrument.exp_time.as_ref().unwrap_or(&"N/A".to_string()),
                    instrument.alias.as_ref().unwrap_or(&"N/A".to_string())
                );
            }
        }
        Err(e) => eprintln!("Error getting BTC-USD FUTURES: {:?}", e),
    }

    Ok(())
}