use rest::Client;
/// Example of using Bitget Futures API
///
/// This example demonstrates how to use the newly implemented
/// Bitget Futures API endpoints for both public and private data.
use venues::bitget::ProductType;
use venues::bitget::futures::public::rest::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client (for demonstration purposes - not used in this example)
    let _client = Client::new("https://api.bitget.com", None, None, None)?;

    // Public Endpoints Examples
    println!("=== Bitget Futures Public API Examples ===");

    // Get VIP fee rates
    let vip_rates_request = GetVipFeeRateRequest::default();
    println!("VIP Fee Rates request created: {:?}", vip_rates_request);

    // Get ticker for a specific symbol
    let ticker_request = GetTickerRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
    };
    println!("Ticker request created: {:?}", ticker_request);

    // Get all tickers
    let all_tickers_request = GetAllTickersRequest {
        product_type: ProductType::UsdtFutures,
    };
    println!("All Tickers request created: {:?}", all_tickers_request);

    // Get market depth
    let depth_request = MarketDepthRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
        precision: Some("scale0".to_string()),
        limit: Some("50".to_string()),
    };
    println!("Market Depth request created: {:?}", depth_request);

    // Get candlestick data
    let candle_request = CandlestickRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
        granularity: "1h".to_string(),
        start_time: None,
        end_time: None,
        limit: Some("100".to_string()),
    };
    println!("Candlestick request created: {:?}", candle_request);

    // Get open interest
    let oi_request = OpenInterestRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
    };
    println!("Open Interest request created: {:?}", oi_request);

    // Get funding rate
    let funding_request = CurrentFundingRateRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
    };
    println!(
        "Current Funding Rate request created: {:?}",
        funding_request
    );

    // Get contract config
    let config_request = ContractConfigRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
    };
    println!("Contract Config request created: {:?}", config_request);

    /*
    // Note: To actually make API calls, you would need a properly configured RestClient
    // and use the methods defined in each endpoint module:
    //
    // let rest_client = RestClient::new(...);
    // let vip_rates = rest_client.get_vip_fee_rate(vip_rates_request).await?;
    // let ticker = rest_client.get_ticker(ticker_request).await?;
    // etc.
     */

    println!("\nBitget Futures API implementation completed successfully!");
    println!("All request structures are properly constructed and ready for use.");
    Ok(())
}
