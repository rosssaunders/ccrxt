use venues::gateio::{
    CandlestickInterval, Result,
    public::rest::{
        RestClient, candlesticks::CandlesticksRequest, order_book::OrderBookRequest,
        tickers::TickersRequest, trades::TradesRequest,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a new public REST client (using live environment)
    let client = RestClient::new(false)?;

    println!("Gate.io Market Data Example");
    println!("===========================");

    // Get all currencies
    println!("\n📜 Fetching all currencies...");
    match client.list_currencies().await {
        Ok(currencies) => {
            println!("✅ Found {} currencies", currencies.len());
            // Show first 5 currencies
            for currency in currencies.iter().take(5) {
                println!(
                    "  💰 {}: delisted={}, withdraw_disabled={}",
                    currency.currency, currency.delisted, currency.withdraw_disabled
                );
            }
        }
        Err(e) => println!("❌ Error fetching currencies: {}", e),
    }

    // Get all trading pairs
    println!("\n📊 Fetching all trading pairs...");
    match client.list_currency_pairs().await {
        Ok(pairs) => {
            println!("✅ Found {} trading pairs", pairs.len());
            // Show first 5 pairs
            for pair in pairs.iter().take(5) {
                println!(
                    "  📈 {}: fee={}, status={}",
                    pair.id, pair.fee, pair.trade_status
                );
            }
        }
        Err(e) => println!("❌ Error fetching trading pairs: {}", e),
    }

    // Get ticker for BTC_USDT
    println!("\n🎯 Fetching ticker for BTC_USDT...");
    let ticker_request = TickersRequest {
        currency_pair: Some("BTC_USDT".to_string()),
        timezone: None,
    };
    match client.get_tickers(ticker_request).await {
        Ok(tickers) => {
            if let Some(ticker) = tickers.first() {
                println!("✅ BTC_USDT Ticker:");
                println!("  💰 Last price: {}", ticker.last);
                println!("  📈 24h high: {}", ticker.high_24h);
                println!("  📉 24h low: {}", ticker.low_24h);
                println!("  📊 24h volume: {}", ticker.base_volume);
                println!("  📈 24h change: {}%", ticker.change_percentage);
            }
        }
        Err(e) => println!("❌ Error fetching ticker: {}", e),
    }

    // Get order book for BTC_USDT
    println!("\n📖 Fetching order book for BTC_USDT...");
    let orderbook_request = OrderBookRequest {
        currency_pair: "BTC_USDT".to_string(),
        limit: Some(5),
        with_id: None,
    };
    match client.get_order_book(orderbook_request).await {
        Ok(orderbook) => {
            println!("✅ Order Book (top 5 levels):");
            println!("  📊 Asks (sell orders):");
            for ask in orderbook.asks.iter().take(5) {
                if ask.len() >= 2 {
                    println!("    🔴 {} @ {}", ask[1], ask[0]);
                }
            }
            println!("  📊 Bids (buy orders):");
            for bid in orderbook.bids.iter().take(5) {
                if bid.len() >= 2 {
                    println!("    🟢 {} @ {}", bid[1], bid[0]);
                }
            }
        }
        Err(e) => println!("❌ Error fetching order book: {}", e),
    }

    // Get recent trades for BTC_USDT
    println!("\n💱 Fetching recent trades for BTC_USDT...");
    let trades_request = TradesRequest {
        currency_pair: "BTC_USDT".to_string(),
        limit: Some(5),
        page: None,
        from: None,
        to: None,
    };
    match client.get_trades(trades_request).await {
        Ok(trades) => {
            println!("✅ Recent trades (last 5):");
            for trade in trades.iter().take(5) {
                println!(
                    "  {} {} {} @ {} ({})",
                    match trade.side.as_str() {
                        "buy" => "🟢",
                        "sell" => "🔴",
                        _ => "❓",
                    },
                    trade.side,
                    trade.amount,
                    trade.price,
                    trade.create_time
                );
            }
        }
        Err(e) => println!("❌ Error fetching trades: {}", e),
    }

    // Get candlestick data for BTC_USDT
    println!("\n🕯️ Fetching 1-hour candlesticks for BTC_USDT...");
    let candlesticks_request = CandlesticksRequest {
        currency_pair: "BTC_USDT".to_string(),
        interval: CandlestickInterval::Hours1,
        limit: Some(5),
        from: None,
        to: None,
    };
    match client.get_candlesticks(candlesticks_request).await {
        Ok(candlesticks) => {
            println!("✅ Recent candlesticks (last 5):");
            for candle in candlesticks.iter().take(5) {
                if candle.len() >= 7 {
                    println!(
                        "  🕯️ O:{} H:{} L:{} C:{} V:{}",
                        candle[5], // open
                        candle[3], // high
                        candle[4], // low
                        candle[2], // close
                        candle[1]  // volume
                    );
                }
            }
        }
        Err(e) => println!("❌ Error fetching candlesticks: {}", e),
    }

    println!("\n✅ Gate.io market data example completed!");
    Ok(())
}
