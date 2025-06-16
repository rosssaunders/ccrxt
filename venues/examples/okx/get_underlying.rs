/// Example of using OKX public API to get underlying assets
///
/// This example demonstrates how to retrieve underlying assets from OKX exchange
/// using the /api/v5/public/underlying endpoint.
use venues::okx::{GetUnderlyingRequest, InstrumentType, PublicRestClient, RateLimiter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client and rate limiter
    let http_client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();

    // Create OKX public REST client
    let client = PublicRestClient::new("https://www.okx.com", http_client, rate_limiter);

    // Example 1: Get underlying assets for SWAP instruments
    println!("=== Getting underlying assets for SWAP instruments ===");
    let swap_request = GetUnderlyingRequest {
        inst_type: InstrumentType::Swap,
    };

    match client.get_underlying(swap_request).await {
        | Ok(response) => {
            println!("Response code: {}", response.code);
            if let Some(data) = response.data.first() {
                println!(
                    "Found {} underlying assets for SWAP instruments:",
                    data.uly.len()
                );

                // Show first few underlying assets
                for (i, underlying) in data.uly.iter().take(10).enumerate() {
                    println!("  {}. {}", i + 1, underlying);
                }

                if data.uly.len() > 10 {
                    println!("  ... and {} more", data.uly.len() - 10);
                }
            }
        },
        | Err(e) => eprintln!("Error getting SWAP underlying assets: {:?}", e),
    }

    // Example 2: Get underlying assets for FUTURES instruments
    println!("\n=== Getting underlying assets for FUTURES instruments ===");
    let futures_request = GetUnderlyingRequest {
        inst_type: InstrumentType::Futures,
    };

    match client.get_underlying(futures_request).await {
        | Ok(response) => {
            if let Some(data) = response.data.first() {
                println!(
                    "Found {} underlying assets for FUTURES instruments:",
                    data.uly.len()
                );

                // Show BTC-related underlying assets
                let btc_underlying: Vec<_> =
                    data.uly.iter().filter(|u| u.contains("BTC")).collect();

                if !btc_underlying.is_empty() {
                    println!("BTC-related underlying assets:");
                    for underlying in btc_underlying {
                        println!("  - {}", underlying);
                    }
                }

                // Show ETH-related underlying assets
                let eth_underlying: Vec<_> = data
                    .uly
                    .iter()
                    .filter(|u| u.contains("ETH"))
                    .take(5)
                    .collect();

                if !eth_underlying.is_empty() {
                    println!("ETH-related underlying assets (first 5):");
                    for underlying in eth_underlying {
                        println!("  - {}", underlying);
                    }
                }
            }
        },
        | Err(e) => eprintln!("Error getting FUTURES underlying assets: {:?}", e),
    }

    // Example 3: Get underlying assets for OPTION instruments
    println!("\n=== Getting underlying assets for OPTION instruments ===");
    let option_request = GetUnderlyingRequest {
        inst_type: InstrumentType::Option,
    };

    match client.get_underlying(option_request).await {
        | Ok(response) => {
            if let Some(data) = response.data.first() {
                println!(
                    "Found {} underlying assets for OPTION instruments:",
                    data.uly.len()
                );

                // Show all underlying assets for options (usually fewer)
                for (i, underlying) in data.uly.iter().enumerate() {
                    println!("  {}. {}", i + 1, underlying);
                }
            }
        },
        | Err(e) => eprintln!("Error getting OPTION underlying assets: {:?}", e),
    }

    Ok(())
}
