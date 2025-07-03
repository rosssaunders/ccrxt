/// BingX Market Data Example
///
/// This example demonstrates how to fetch various types of market data
/// from the BingX public API, including klines, order book, and ticker information.
use reqwest::Client;
use venues::bingx::{
    public::PublicRestClient, Get24hrTickerRequest, GetKlineRequest, GetOrderBookRequest,
    GetRecentTradesRequest, Interval, RateLimiter,
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

    let symbol = "BTC-USDT";
    let current_time = chrono::Utc::now().timestamp_millis();

    println!("BingX Market Data for {}", symbol);
    println!("==============================");

    // 1. Get 24hr ticker data
    println!("\n1. 24-Hour Price Statistics:");
    let ticker_request = Get24hrTickerRequest::for_symbol(symbol.to_string(), current_time);
    
    match client.get_24hr_ticker(&ticker_request).await {
        Ok(response) => {
            if let Some(ticker) = response.tickers.first() {
                println!("  Symbol: {}", ticker.symbol);
                println!("  Last Price: ${}", ticker.last_price);
                println!("  24h Change: {}%", ticker.price_change_percent);
                println!("  24h High: ${}", ticker.high_price);
                println!("  24h Low: ${}", ticker.low_price);
                println!("  24h Volume: {}", ticker.volume);
                println!("  24h Quote Volume: ${:.2}", ticker.quote_volume.parse::<f64>().unwrap_or(0.0));
                println!("  Trade Count: {}", ticker.count);
                println!("  Best Bid: ${} ({})", ticker.bid_price, ticker.bid_qty);
                println!("  Best Ask: ${} ({})", ticker.ask_price, ticker.ask_qty);
            }
        }
        Err(e) => {
            println!("  Error: {:?}", e);
        }
    }

    // 2. Get order book
    println!("\n2. Order Book (Top 10):");
    let order_book_request = GetOrderBookRequest::new(symbol.to_string(), current_time)
        .with_limit(10);
    
    match client.get_order_book(&order_book_request).await {
        Ok(response) => {
            println!("  Timestamp: {}", response.ts);
            println!("  Bids (Price, Quantity):");
            for (i, bid) in response.bids.iter().take(5).enumerate() {
                println!("    {}: ${:.2} @ {:.6}", i + 1, bid[0], bid[1]);
            }
            println!("  Asks (Price, Quantity):");
            for (i, ask) in response.asks.iter().take(5).enumerate() {
                println!("    {}: ${:.2} @ {:.6}", i + 1, ask[0], ask[1]);
            }
        }
        Err(e) => {
            println!("  Error: {:?}", e);
        }
    }

    // 3. Get recent trades
    println!("\n3. Recent Trades (Last 10):");
    let trades_request = GetRecentTradesRequest::new(symbol.to_string(), current_time)
        .with_limit(10);
    
    match client.get_recent_trades(&trades_request).await {
        Ok(response) => {
            for (i, trade) in response.trades.iter().take(10).enumerate() {
                let trade_time = chrono::DateTime::from_timestamp_millis(trade.time)
                    .map(|dt| dt.format("%H:%M:%S").to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                let side = if trade.buyer_maker { "SELL" } else { "BUY" };
                println!("    {}: {} {} @ ${:.2} [{}] - ID: {}", 
                    i + 1, side, trade.qty, trade.price, trade_time, trade.id);
            }
        }
        Err(e) => {
            println!("  Error: {:?}", e);
        }
    }

    // 4. Get different kline intervals
    println!("\n4. Kline/Candlestick Data:");
    
    let intervals = [
        (Interval::OneMinute, "1 Minute"),
        (Interval::FiveMinutes, "5 Minutes"),
        (Interval::OneHour, "1 Hour"),
        (Interval::OneDay, "1 Day"),
    ];

    for (interval, interval_name) in &intervals {
        println!("  {} Candles (Last 3):", interval_name);
        let kline_request = GetKlineRequest::new(symbol.to_string(), *interval, current_time)
            .with_limit(3);
        
        match client.get_kline(&kline_request).await {
            Ok(response) => {
                for (i, kline) in response.klines.iter().enumerate() {
                    let open_time = chrono::DateTime::from_timestamp_millis(kline[0] as i64)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "N/A".to_string());
                    println!("    {}: {} | O: {:.2} H: {:.2} L: {:.2} C: {:.2} V: {:.4}", 
                        i + 1, open_time, kline[1], kline[2], kline[3], kline[4], kline[5]);
                }
            }
            Err(e) => {
                println!("    Error: {:?}", e);
            }
        }
        println!();
    }

    // 5. Price analysis
    println!("5. Quick Price Analysis:");
    
    // Get hourly data for trend analysis
    let hourly_request = GetKlineRequest::new(symbol.to_string(), Interval::OneHour, current_time)
        .with_limit(24); // Last 24 hours
    
    match client.get_kline(&hourly_request).await {
        Ok(response) => {
            if response.klines.len() >= 2 {
                let latest = response.klines.last().unwrap();
                let previous = &response.klines[response.klines.len() - 2];
                
                let current_price = latest[4]; // close price
                let previous_price = previous[4];
                let price_change = current_price - previous_price;
                let price_change_pct = (price_change / previous_price) * 100.0;
                
                // Calculate 24h high/low
                let (high_24h, low_24h) = response.klines.iter()
                    .fold((f64::MIN, f64::MAX), |(max_high, min_low), kline| {
                        (max_high.max(kline[2]), min_low.min(kline[3]))
                    });
                
                // Calculate average volume
                let avg_volume: f64 = response.klines.iter()
                    .map(|kline| kline[5])
                    .sum::<f64>() / response.klines.len() as f64;
                
                println!("  Current Price: ${:.2}", current_price);
                println!("  1h Change: ${:.2} ({:.2}%)", price_change, price_change_pct);
                println!("  24h High: ${:.2}", high_24h);
                println!("  24h Low: ${:.2}", low_24h);
                println!("  24h Range: ${:.2}", high_24h - low_24h);
                println!("  Avg Volume (24h): {:.4}", avg_volume);
                
                let trend = if price_change_pct > 0.5 {
                    "📈 Strong Uptrend"
                } else if price_change_pct > 0.0 {
                    "📊 Slight Uptrend"
                } else if price_change_pct > -0.5 {
                    "📊 Slight Downtrend"
                } else {
                    "📉 Strong Downtrend"
                };
                println!("  Trend: {}", trend);
            }
        }
        Err(e) => {
            println!("  Error: {:?}", e);
        }
    }

    println!("\nMarket data analysis complete!");
    Ok(())
}
