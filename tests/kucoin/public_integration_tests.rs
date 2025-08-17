//! Integration tests for Kucoin public REST API endpoints.
//!
//! These tests verify that the Kucoin public REST API client can successfully
//! communicate with the live API and receive valid responses.

use std::sync::Arc;

use rest::native::NativeHttpClient;
use venues::kucoin::spot::{
    GetAllCurrenciesRequest, GetAllSymbolsRequest, GetAllTickersRequest, GetCurrencyRequest,
    GetKlinesRequest, GetPartOrderBookRequest, GetServerTimeRequest, GetSymbolRequest,
    GetTickerRequest, GetTradesRequest, KlineInterval, OrderBookLevel, PublicRestClient,
    RateLimiter,
};

/// Helper function to create a test client with shared rate limiter
fn create_public_test_client() -> PublicRestClient {
    let rate_limiter = RateLimiter::new();
    let http_client = Arc::new(NativeHttpClient::default());
    PublicRestClient::new("https://api.kucoin.com", rate_limiter, http_client)
}

#[tokio::test]
async fn test_get_all_currencies() {
    let client = create_public_test_client();
    let request = GetAllCurrenciesRequest {};

    let result = client.get_all_currencies(request).await;
    assert!(
        result.is_ok(),
        "get_all_currencies should succeed: {:?}",
        result.err()
    );

    let (currencies, _headers) = result.unwrap();
    assert!(!currencies.is_empty(), "Should have at least one currency");

    println!("Found {} currencies", currencies.len());
    if let Some(first_currency) = currencies.first() {
        println!(
            "First currency: {} ({})",
            first_currency.currency, first_currency.full_name
        );
    }
}

#[tokio::test]
async fn test_get_currency() {
    let client = create_public_test_client();
    let request = GetCurrencyRequest {
        currency: "BTC".to_string(),
    };

    let result = client.get_currency(request).await;
    assert!(
        result.is_ok(),
        "get_currency should succeed: {:?}",
        result.err()
    );

    let (currency, _headers) = result.unwrap();

    println!("Currency: {} - {}", currency.currency, currency.full_name);
    println!(
        "Precision: {}, Margin enabled: {}",
        currency.precision, currency.is_margin_enabled
    );
}

#[tokio::test]
async fn test_get_all_symbols() {
    let client = create_public_test_client();
    let request = GetAllSymbolsRequest { market: None };

    let result = client.get_all_symbols(request).await;
    assert!(
        result.is_ok(),
        "get_all_symbols should succeed: {:?}",
        result.err()
    );

    let (symbols, _headers) = result.unwrap();
    assert!(!symbols.is_empty(), "Should have at least one symbol");

    println!("Found {} symbols", symbols.len());
    if let Some(first_symbol) = symbols.first() {
        println!(
            "First symbol: {} ({}/{})",
            first_symbol.symbol, first_symbol.base_currency, first_symbol.quote_currency
        );
    }
}

#[tokio::test]
async fn test_get_symbol() {
    let client = create_public_test_client();
    let request = GetSymbolRequest {
        symbol: "BTC-USDT".to_string(),
    };

    let result = client.get_symbol(request).await;
    assert!(
        result.is_ok(),
        "get_symbol should succeed: {:?}",
        result.err()
    );

    let (symbol, _headers) = result.unwrap();

    println!(
        "Symbol: {} ({}/{})",
        symbol.symbol, symbol.base_currency, symbol.quote_currency
    );
    println!(
        "Market: {}, Fee currency: {}",
        symbol.market, symbol.fee_currency
    );
}

#[tokio::test]
async fn test_get_all_tickers() {
    let client = create_public_test_client();
    let request = GetAllTickersRequest {};

    let result = client.get_all_tickers(request).await;
    assert!(
        result.is_ok(),
        "get_all_tickers should succeed: {:?}",
        result.err()
    );

    let (tickers, _headers) = result.unwrap();
    assert!(
        !tickers.ticker.is_empty(),
        "Should have at least one ticker"
    );

    println!("Server time: {}", tickers.time);
    println!("Found {} tickers", tickers.ticker.len());
    if let Some(first_ticker) = tickers.ticker.first() {
        println!(
            "First ticker: {} - Last: {}, Volume: {}",
            first_ticker.symbol,
            first_ticker.last_price.as_deref().unwrap_or("N/A"),
            first_ticker.vol.as_deref().unwrap_or("N/A")
        );
    }
}

#[tokio::test]
async fn test_get_ticker() {
    let client = create_public_test_client();
    let request = GetTickerRequest {
        symbol: "BTC-USDT".to_string(),
    };

    let result = client.get_ticker(request).await;
    assert!(
        result.is_ok(),
        "get_ticker should succeed: {:?}",
        result.err()
    );

    let (ticker, _headers) = result.unwrap();

    println!(
        "Ticker for {}: Last: {}, High: {}, Low: {}",
        ticker.symbol, ticker.last, ticker.high, ticker.low
    );
}

#[tokio::test]
async fn test_get_part_order_book() {
    let client = create_public_test_client();
    let request = GetPartOrderBookRequest {
        symbol: "BTC-USDT".to_string(),
        level: OrderBookLevel::Twenty,
    };

    let result = client.get_part_order_book(request).await;
    assert!(
        result.is_ok(),
        "get_part_order_book should succeed: {:?}",
        result.err()
    );

    let (orderbook, _headers) = result.unwrap();

    println!("Order book sequence: {}", orderbook.sequence);
    println!(
        "Asks: {} levels, Bids: {} levels",
        orderbook.asks.len(),
        orderbook.bids.len()
    );

    if let Some(best_ask) = orderbook.asks.first() {
        println!("Best ask: {} @ {}", best_ask[1], best_ask[0]);
    }
    if let Some(best_bid) = orderbook.bids.first() {
        println!("Best bid: {} @ {}", best_bid[1], best_bid[0]);
    }
}

#[tokio::test]
async fn test_get_trades() {
    let client = create_public_test_client();
    let request = GetTradesRequest {
        symbol: "BTC-USDT".to_string(),
    };

    let result = client.get_trades(request).await;
    assert!(
        result.is_ok(),
        "get_trades should succeed: {:?}",
        result.err()
    );

    let (trades, _headers) = result.unwrap();
    assert!(!trades.is_empty(), "Should have at least one trade");

    println!("Found {} recent trades", trades.len());
    if let Some(first_trade) = trades.first() {
        println!(
            "First trade: {:?} {} @ {} at {}",
            first_trade.side, first_trade.size, first_trade.price, first_trade.time
        );
    }
}

#[tokio::test]
async fn test_get_klines() {
    let client = create_public_test_client();

    // Get 1-minute klines for the last hour
    let end_time = chrono::Utc::now().timestamp();
    let start_time = end_time - 3600; // 1 hour ago

    let request = GetKlinesRequest {
        symbol: "BTC-USDT".to_string(),
        interval: KlineInterval::OneMinute,
        start_time: Some(start_time),
        end_time: Some(end_time),
    };

    let result = client.get_klines(request).await;
    assert!(
        result.is_ok(),
        "get_klines should succeed: {:?}",
        result.err()
    );

    let (klines, _headers) = result.unwrap();
    assert!(!klines.is_empty(), "Should have at least one kline");

    println!("Found {} klines", klines.len());
    if let Some(first_kline) = klines.first() {
        println!(
            "First kline: Time: {}, Open: {}, High: {}, Low: {}, Close: {}",
            first_kline.open_time,
            first_kline.open,
            first_kline.high,
            first_kline.low,
            first_kline.close
        );
    }
}

#[tokio::test]
async fn test_get_server_time() {
    let client = create_public_test_client();
    let request = GetServerTimeRequest {};

    let result = client.get_server_time(request).await;
    assert!(
        result.is_ok(),
        "get_server_time should succeed: {:?}",
        result.err()
    );

    let (server_time_response, _headers) = result.unwrap();

    let server_time = server_time_response.timestamp;
    println!("Server timestamp: {}", server_time);

    // Convert to human-readable format
    use chrono::{DateTime, Utc};
    let datetime = DateTime::<Utc>::from_timestamp_millis(server_time);
    if let Some(dt) = datetime {
        println!("Server time: {}", dt.format("%Y-%m-%d %H:%M:%S UTC"));
    }
}
