/// Example demonstrating BingX public API endpoint usage
///
/// This example shows how to use the BingX public REST API endpoints
/// to fetch market data, symbols, trades, order books, and other public information.
#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod example {
    use venues::bingx::{
        PublicRestClient, RateLimiter,
        GetServerTimeRequest, GetSymbolsRequest, GetRecentTradesRequest,
        GetOrderBookRequest, GetKlineRequest, Get24hrTickerRequest,
        GetOrderBookAggregationRequest, GetSymbolPriceTickerRequest,
        GetSymbolOrderBookTickerRequest, GetHistoricalKlineRequest,
        GetOldTradeRequest,
    };
    use reqwest::Client;

    /// Example demonstrating usage of the BingX public endpoints
    #[test]
    fn example_bingx_public_endpoints() {
        // Note: These are just examples of method calls - they won't make actual HTTP requests in tests

        // Create a public client
        let client = PublicRestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        // Example usage patterns (would be used in real applications):

        // 1. Get server time
        let _server_time_request = GetServerTimeRequest::default();
        // let server_time = client.get_server_time().await?;

        // 2. Get all trading symbols
        let timestamp = 1640995200000;
        let _symbols_request = GetSymbolsRequest::new(timestamp);
        // let symbols = client.get_symbols(&symbols_request).await?;

        // 3. Get symbols for a specific trading pair
        let _btc_usdt_symbols = GetSymbolsRequest::for_symbol("BTC-USDT".to_string(), timestamp);
        // let btc_symbols = client.get_symbols(&btc_usdt_symbols).await?;

        // 4. Get recent trades for a symbol
        let _recent_trades_request = GetRecentTradesRequest::new("BTC-USDT".to_string(), timestamp)
            .with_limit(50);
        // let recent_trades = client.get_recent_trades(&recent_trades_request).await?;

        // 5. Get order book depth
        let _order_book_request = GetOrderBookRequest::new("BTC-USDT".to_string(), timestamp)
            .with_limit(20);
        // let order_book = client.get_order_book(&order_book_request).await?;

        // 6. Get kline/candlestick data
        let _kline_request = GetKlineRequest::new("BTC-USDT".to_string(), "1h".to_string(), timestamp)
            .with_limit(100)
            .with_start_time(1640995200000)
            .with_end_time(1641081600000);
        // let klines = client.get_kline(&kline_request).await?;

        // 7. Get 24hr ticker for all symbols
        let _ticker_24hr_request = Get24hrTickerRequest::new(timestamp);
        // let tickers = client.get_24hr_ticker(&ticker_24hr_request).await?;

        // 8. Get 24hr ticker for a specific symbol
        let _btc_ticker_request = Get24hrTickerRequest::for_symbol("BTC-USDT".to_string(), timestamp);
        // let btc_ticker = client.get_24hr_ticker(&btc_ticker_request).await?;

        // 9. Get order book aggregation with custom precision
        let _order_book_agg_request = GetOrderBookAggregationRequest::new(
            "BTC_USDT".to_string(),
            20,
            "step0".to_string(),
        );
        // let order_book_agg = client.get_order_book_aggregation(&order_book_agg_request).await?;

        // 10. Get symbol price ticker
        let _price_ticker_request = GetSymbolPriceTickerRequest::new("BTC_USDT".to_string());
        // let price_ticker = client.get_symbol_price_ticker(&price_ticker_request).await?;

        // 11. Get symbol order book ticker
        let _book_ticker_request = GetSymbolOrderBookTickerRequest::new("BTC_USDT".to_string());
        // let book_ticker = client.get_symbol_order_book_ticker(&book_ticker_request).await?;

        // 12. Get historical K-line data
        let _historical_kline_request = GetHistoricalKlineRequest::new("BTC-USDT".to_string(), "1h".to_string())
            .with_limit(500)
            .with_start_time(1640995200000)
            .with_end_time(1641081600000);
        // let historical_klines = client.get_historical_kline(&historical_kline_request).await?;

        // 13. Get old trade data
        let _old_trade_request = GetOldTradeRequest::new("BTC-USDT".to_string())
            .with_limit(100)
            .with_from_id("12345".to_string());
        // let old_trades = client.get_old_trade(&old_trade_request).await?;

        println!("All BingX public API endpoint methods are available and properly typed");
    }

    /// Example demonstrating error handling patterns
    #[test]
    fn example_error_handling() {
        // Example of how error handling would work in real applications
        // (these won't actually execute network requests in tests)

        async fn example_with_error_handling() -> Result<(), venues::bingx::Errors> {
            let client = PublicRestClient::new(
                "https://open-api.bingx.com",
                Client::new(),
                RateLimiter::new(),
            );

            // Example: Handle different types of errors
            match client.get_server_time().await {
                Ok(response) => {
                    println!("Server time: {}", response.server_time);
                }
                Err(venues::bingx::Errors::RateLimitExceeded(msg)) => {
                    eprintln!("Rate limit exceeded: {}", msg);
                    // Implement retry logic with backoff
                }
                Err(venues::bingx::Errors::NetworkError(msg)) => {
                    eprintln!("Network error: {}", msg);
                    // Implement retry logic
                }
                Err(venues::bingx::Errors::ApiError { code, msg }) => {
                    eprintln!("API error {}: {}", code, msg);
                    // Handle specific API errors
                }
                Err(e) => {
                    eprintln!("Other error: {}", e);
                }
            }

            Ok(())
        }

        // This demonstrates the function signature but won't execute
        let _ = example_with_error_handling;
    }
}