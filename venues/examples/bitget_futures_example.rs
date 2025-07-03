/// Example of using Bitget Futures API
/// 
/// This example demonstrates how to use the newly implemented 
/// Bitget Futures API endpoints for both public and private data.

use venues::bitget::enums::*;
use venues::bitget::futures::public::rest::*;
use venues::bitget::futures::private::rest::*;
use rest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = Client::new("https://api.bitget.com", None, None, None)?;

    // Public Endpoints Examples
    println!("=== Bitget Futures Public API Examples ===");

    // Get VIP fee rates
    let vip_rates = vip_fee_rate(&client, None).await?;
    println!("VIP Fee Rates: {:?}", vip_rates);

    // Get ticker for a specific symbol
    let ticker_request = GetTickerRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
    };
    let ticker = ticker(&client, &ticker_request).await?;
    println!("Ticker: {:?}", ticker);

    // Get all tickers
    let all_tickers_request = GetAllTickersRequest {
        product_type: ProductType::UsdtFutures,
    };
    let all_tickers = all_tickers(&client, &all_tickers_request).await?;
    println!("All Tickers count: {}", all_tickers.len());

    // Get market depth
    let depth_request = MarketDepthRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
        precision: Some("scale0".to_string()),
        limit: Some("50".to_string()),
    };
    let depth = market_depth(&client, &depth_request).await?;
    println!("Market Depth: bids={}, asks={}", depth.bids.len(), depth.asks.len());

    // Get candlestick data
    let candle_request = CandlestickRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
        granularity: "1h".to_string(),
        start_time: None,
        end_time: None,
        limit: Some("100".to_string()),
    };
    let candles = candlestick(&client, &candle_request).await?;
    println!("Candlestick data count: {}", candles.len());

    // Get open interest
    let oi_request = OpenInterestRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
    };
    let open_interest_data = open_interest(&client, &oi_request).await?;
    println!("Open Interest: {:?}", open_interest_data);

    // Get funding rate
    let funding_request = CurrentFundingRateRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
    };
    let funding = current_funding_rate(&client, &funding_request).await?;
    println!("Current Funding Rate: {:?}", funding);

    // Get contract config
    let config_request = ContractConfigRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
    };
    let config = contract_config(&client, &config_request).await?;
    println!("Contract Config: {:?}", config);

    /*
    // Private Endpoints Examples (commented out as they require authentication)
    println!("\n=== Bitget Futures Private API Examples ===");

    // Note: These examples require valid API credentials
    let authenticated_client = Client::new(
        "https://api.bitget.com", 
        Some("your_api_key".to_string()), 
        Some("your_secret".to_string()), 
        Some("your_passphrase".to_string())
    )?;

    // Get account info
    let account_request = GetAccountRequest {
        product_type: ProductType::UsdtFutures,
        symbol: "BTCUSDT".to_string(),
        margin_coin: "USDT".to_string(),
    };
    let account = get_account(&authenticated_client, &account_request).await?;
    println!("Account Info: {:?}", account);

    // Get all positions
    let positions_request = AllPositionsRequest {
        product_type: ProductType::UsdtFutures,
        margin_coin: Some("USDT".to_string()),
    };
    let positions = all_positions(&authenticated_client, &positions_request).await?;
    println!("Positions count: {}", positions.len());

    // Place an order
    let order_request = PlaceOrderRequest {
        symbol: "BTCUSDT".to_string(),
        product_type: ProductType::UsdtFutures,
        margin_coin: MarginCoin::Usdt,
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        size: "0.001".to_string(),
        price: Some("30000".to_string()),
        time_in_force: Some(TimeInForce::Gtc),
        client_oid: None,
        reduce_only: Some(false),
        hold_side: Some(HoldSide::Long),
    };
    let order_result = place_order(&authenticated_client, &order_request).await?;
    println!("Order placed: {:?}", order_result);
    */

    println!("\nBitget Futures API implementation completed successfully!");
    Ok(())
}
