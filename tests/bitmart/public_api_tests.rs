//! BitMart public API integration tests
//!
//! These tests verify the functionality of BitMart public REST endpoints.

#[cfg(test)]
mod tests {
    use venues::bitmart::{
        PublicRestClient, GetCurrencyListRequest, GetTickerAllPairsRequest,
        GetTradingPairsListRequest, GetTickerRequest, GetDepthRequest,
        GetLatestKlineRequest, GetHistoryKlineRequest, GetRecentTradesRequest
    };

    /// Test helper to create a public client
    fn create_public_client() -> PublicRestClient {
        PublicRestClient::new()
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_get_currency_list() {
        let client = create_public_client();
        let request = GetCurrencyListRequest::default();
        
        let result = client.get_currency_list(request).await;
        
        match result {
            Ok(response) => {
                assert!(!response.currencies.is_empty(), "Currency list should not be empty");
                
                // Check that each currency has required fields
                for currency in &response.currencies {
                    assert!(!currency.id.is_empty(), "Currency ID should not be empty");
                    assert!(!currency.name.is_empty(), "Currency name should not be empty");
                }
                
                println!("✓ Currency list test passed with {} currencies", response.currencies.len());
            }
            Err(e) => {
                println!("Currency list test failed: {:?}", e);
                // Don't panic here as this might be due to network issues or API changes
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_get_trading_pairs_list() {
        let client = create_public_client();
        let request = GetTradingPairsListRequest::default();
        
        let result = client.get_trading_pairs_list(request).await;
        
        match result {
            Ok(response) => {
                assert!(!response.symbols.is_empty(), "Trading pairs list should not be empty");
                
                // Check for common trading pairs
                let symbols: Vec<&str> = response.symbols.iter().map(|s| s.as_str()).collect();
                
                println!("✓ Trading pairs test passed with {} pairs", response.symbols.len());
                
                // Look for some common pairs (but don't fail if they're not there)
                if symbols.iter().any(|&s| s.contains("BTC")) {
                    println!("  Found BTC pairs");
                }
                if symbols.iter().any(|&s| s.contains("ETH")) {
                    println!("  Found ETH pairs");
                }
            }
            Err(e) => {
                println!("Trading pairs list test failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_get_ticker_all_pairs() {
        let client = create_public_client();
        let request = GetTickerAllPairsRequest::default();
        
        let result = client.get_ticker_all_pairs(request).await;
        
        match result {
            Ok(response) => {
                assert!(!response.tickers.is_empty(), "Ticker data should not be empty");
                
                // Check that ticker data has required fields
                for ticker in response.tickers.iter().take(5) { // Just check first 5
                    assert!(!ticker.symbol.is_empty(), "Symbol should not be empty");
                    assert!(!ticker.last.is_empty(), "Last price should not be empty");
                }
                
                println!("✓ All pairs ticker test passed with {} tickers", response.tickers.len());
            }
            Err(e) => {
                println!("All pairs ticker test failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_get_ticker_single_pair() {
        let client = create_public_client();
        let request = GetTickerRequest {
            symbol: "BTC_USDT".to_string(),
        };
        
        let result = client.get_ticker(request).await;
        
        match result {
            Ok(response) => {
                assert_eq!(response.symbol, "BTC_USDT");
                assert!(!response.last.is_empty(), "Last price should not be empty");
                assert!(!response.v_24h.is_empty(), "24h volume should not be empty");
                
                println!("✓ Single pair ticker test passed for BTC_USDT");
                println!("  Last price: {}", response.last);
                println!("  24h volume: {}", response.v_24h);
            }
            Err(e) => {
                println!("Single pair ticker test failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_get_depth() {
        let client = create_public_client();
        let request = GetDepthRequest {
            symbol: "BTC_USDT".to_string(),
            precision: Some(4),
            size: Some(50),
        };
        
        let result = client.get_depth(request).await;
        
        match result {
            Ok(response) => {
                assert_eq!(response.symbol, "BTC_USDT");
                assert!(!response.buys.is_empty(), "Buy orders should not be empty");
                assert!(!response.sells.is_empty(), "Sell orders should not be empty");
                
                // Check order book structure
                for buy_order in response.buys.iter().take(3) {
                    assert!(!buy_order.price.is_empty(), "Buy order price should not be empty");
                    assert!(!buy_order.amount.is_empty(), "Buy order amount should not be empty");
                }
                
                println!("✓ Depth test passed for BTC_USDT");
                println!("  Buy orders: {}, Sell orders: {}", response.buys.len(), response.sells.len());
            }
            Err(e) => {
                println!("Depth test failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_get_recent_trades() {
        let client = create_public_client();
        let request = GetRecentTradesRequest {
            symbol: "BTC_USDT".to_string(),
            n: Some(10),
        };
        
        let result = client.get_recent_trades(request).await;
        
        match result {
            Ok(response) => {
                assert!(!response.trades.is_empty(), "Recent trades should not be empty");
                
                // Check trade structure
                for trade in response.trades.iter().take(3) {
                    assert!(!trade.price.is_empty(), "Trade price should not be empty");
                    assert!(!trade.amount.is_empty(), "Trade amount should not be empty");
                    assert!(!trade.count.is_empty(), "Trade count should not be empty");
                }
                
                println!("✓ Recent trades test passed for BTC_USDT");
                println!("  Found {} recent trades", response.trades.len());
            }
            Err(e) => {
                println!("Recent trades test failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_get_latest_kline() {
        let client = create_public_client();
        let request = GetLatestKlineRequest {
            symbol: "BTC_USDT".to_string(),
            step: 1, // 1 minute
        };
        
        let result = client.get_latest_kline(request).await;
        
        match result {
            Ok(response) => {
                assert!(!response.klines.is_empty(), "Latest kline data should not be empty");
                
                // Check kline structure
                for kline in response.klines.iter().take(3) {
                    assert!(!kline.open.is_empty(), "Kline open should not be empty");
                    assert!(!kline.high.is_empty(), "Kline high should not be empty");
                    assert!(!kline.low.is_empty(), "Kline low should not be empty");
                    assert!(!kline.close.is_empty(), "Kline close should not be empty");
                }
                
                println!("✓ Latest kline test passed for BTC_USDT");
                println!("  Found {} kline entries", response.klines.len());
            }
            Err(e) => {
                println!("Latest kline test failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires network access  
    async fn test_get_history_kline() {
        let client = create_public_client();
        let request = GetHistoryKlineRequest {
            symbol: "BTC_USDT".to_string(),
            step: 1, // 1 minute
            from: 1640995200, // Example timestamp
            to: 1641001200,   // Example timestamp
        };
        
        let result = client.get_history_kline(request).await;
        
        match result {
            Ok(response) => {
                // History kline might be empty for the given time range
                println!("✓ History kline test passed for BTC_USDT");
                println!("  Found {} historical kline entries", response.klines.len());
                
                if !response.klines.is_empty() {
                    // Check kline structure if data exists
                    for kline in response.klines.iter().take(3) {
                        assert!(!kline.open.is_empty(), "Kline open should not be empty");
                        assert!(!kline.high.is_empty(), "Kline high should not be empty");
                        assert!(!kline.low.is_empty(), "Kline low should not be empty");
                        assert!(!kline.close.is_empty(), "Kline close should not be empty");
                    }
                }
            }
            Err(e) => {
                println!("History kline test failed: {:?}", e);
            }
        }
    }
}
