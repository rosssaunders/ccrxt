/// BingX Public REST API Example
///
/// This example demonstrates how to use the BingX public REST API endpoints
/// to fetch market data. No API keys are required for public endpoints.
use reqwest::Client;
use venues::bingx::{
    public::PublicRestClient, DepthType, EndpointType, Get24hrTickerRequest,
    GetHistoricalKlineRequest, GetKlineRequest, GetOrderBookAggregationRequest,
    GetOrderBookRequest, GetRecentTradesRequest, GetServerTimeRequest,
    GetSymbolOrderBookTickerRequest, GetSymbolPriceTickerRequest, GetSymbolsRequest, Interval,
    RateLimiter,
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

    println!("BingX Public API Examples");
    println!("========================");

    // Example 1: Get server time
    println!("\n1. Getting server time...");
    let server_time_request = GetServerTimeRequest::new();
    match client.get_server_time(&server_time_request).await {
        Ok(response) => {
            println!("Server time: {}", response.server_time);
        }
        Err(e) => {
            println!("Error getting server time: {:?}", e);
        }
    }

    // Example 2: Get trading symbols
    println!("\n2. Getting trading symbols...");
    let symbols_request = GetSymbolsRequest::new(chrono::Utc::now().timestamp_millis());
    match client.get_symbols(&symbols_request).await {
        Ok(response) => {
            println!("Found {} symbols", response.symbols.len());
            if let Some(symbol) = response.symbols.first() {
                println!("First symbol: {} (status: {:?})", symbol.symbol, symbol.status);
            }
        }
        Err(e) => {
            println!("Error getting symbols: {:?}", e);
        }
    }

    // Example 3: Get recent trades for BTC-USDT
    println!("\n3. Getting recent trades for BTC-USDT...");
    let trades_request = GetRecentTradesRequest::new(
        "BTC-USDT".to_string(),
        chrono::Utc::now().timestamp_millis(),
    )
    .with_limit(5);
    match client.get_recent_trades(&trades_request).await {
        Ok(response) => {
            println!("Recent trades: {}", response.trades.len());
            for trade in response.trades.iter().take(3) {
                println!("  Trade ID: {}, Price: {}, Qty: {}", trade.id, trade.price, trade.qty);
            }
        }
        Err(e) => {
            println!("Error getting recent trades: {:?}", e);
        }
    }

    // Example 4: Get order book
    println!("\n4. Getting order book for BTC-USDT...");
    let order_book_request = GetOrderBookRequest::new(
        "BTC-USDT".to_string(),
        chrono::Utc::now().timestamp_millis(),
    )
    .with_limit(5);
    match client.get_order_book(&order_book_request).await {
        Ok(response) => {
            println!("Order book timestamp: {}", response.ts);
            println!("Bids: {}, Asks: {}", response.bids.len(), response.asks.len());
            if let Some(best_bid) = response.bids.first() {
                println!("Best bid: {} @ {}", best_bid[1], best_bid[0]);
            }
            if let Some(best_ask) = response.asks.first() {
                println!("Best ask: {} @ {}", best_ask[1], best_ask[0]);
            }
        }
        Err(e) => {
            println!("Error getting order book: {:?}", e);
        }
    }

    // Example 5: Get kline/candlestick data
    println!("\n5. Getting 1-hour kline data for BTC-USDT...");
    let kline_request = GetKlineRequest::new(
        "BTC-USDT".to_string(),
        Interval::OneHour,
        chrono::Utc::now().timestamp_millis(),
    )
    .with_limit(5);
    match client.get_kline(&kline_request).await {
        Ok(response) => {
            println!("Klines: {}", response.klines.len());
            for kline in response.klines.iter().take(2) {
                println!(
                    "  Time: {}, Open: {}, High: {}, Low: {}, Close: {}, Volume: {}",
                    kline[0] as i64, kline[1], kline[2], kline[3], kline[4], kline[5]
                );
            }
        }
        Err(e) => {
            println!("Error getting kline data: {:?}", e);
        }
    }

    // Example 6: Get 24hr ticker
    println!("\n6. Getting 24hr ticker for BTC-USDT...");
    let ticker_request = Get24hrTickerRequest::for_symbol(
        "BTC-USDT".to_string(),
        chrono::Utc::now().timestamp_millis(),
    );
    match client.get_24hr_ticker(&ticker_request).await {
        Ok(response) => {
            if let Some(ticker) = response.tickers.first() {
                println!("Symbol: {}", ticker.symbol);
                println!("Last price: {}", ticker.last_price);
                println!("24hr change: {}%", ticker.price_change_percent);
                println!("24hr volume: {}", ticker.volume);
            }
        }
        Err(e) => {
            println!("Error getting 24hr ticker: {:?}", e);
        }
    }

    // Example 7: Get symbol price ticker
    println!("\n7. Getting price ticker for BTC-USDT...");
    let price_ticker_request = GetSymbolPriceTickerRequest::new("BTC-USDT".to_string());
    match client.get_symbol_price_ticker(&price_ticker_request).await {
        Ok(response) => {
            println!("Symbol: {}", response.symbol);
            println!("Price: {}", response.price);
            println!("Timestamp: {}", response.timestamp);
        }
        Err(e) => {
            println!("Error getting price ticker: {:?}", e);
        }
    }

    // Example 8: Get order book ticker
    println!("\n8. Getting order book ticker for BTC-USDT...");
    let book_ticker_request = GetSymbolOrderBookTickerRequest::new("BTC-USDT".to_string());
    match client.get_symbol_order_book_ticker(&book_ticker_request).await {
        Ok(response) => {
            println!("Symbol: {}", response.symbol);
            println!("Best bid: {} @ {}", response.bid_volume, response.bid_price);
            println!("Best ask: {} @ {}", response.ask_volume, response.ask_price);
        }
        Err(e) => {
            println!("Error getting order book ticker: {:?}", e);
        }
    }

    // Example 9: Get order book aggregation
    println!("\n9. Getting order book aggregation for BTC-USDT...");
    let aggregation_request = GetOrderBookAggregationRequest::new(
        "BTC_USDT".to_string(),
        20,
        DepthType::Step0,
    );
    match client.get_order_book_aggregation(&aggregation_request).await {
        Ok(response) => {
            println!("Aggregated order book timestamp: {}", response.ts);
            println!("Bids: {}, Asks: {}", response.bids.len(), response.asks.len());
        }
        Err(e) => {
            println!("Error getting order book aggregation: {:?}", e);
        }
    }

    // Example 10: Get historical kline data
    println!("\n10. Getting historical kline data for BTC-USDT...");
    let historical_kline_request = GetHistoricalKlineRequest::new(
        "BTC-USDT".to_string(),
        Interval::OneDay,
    )
    .with_limit(3);
    match client.get_historical_kline(&historical_kline_request).await {
        Ok(response) => {
            println!("Historical klines: {}", response.klines.len());
            for kline in response.klines.iter().take(2) {
                println!(
                    "  Time: {}, Open: {}, High: {}, Low: {}, Close: {}",
                    kline[0] as i64, kline[1], kline[2], kline[3], kline[4]
                );
            }
        }
        Err(e) => {
            println!("Error getting historical kline data: {:?}", e);
        }
    }

    println!("\nAll examples completed!");
    Ok(())
}
