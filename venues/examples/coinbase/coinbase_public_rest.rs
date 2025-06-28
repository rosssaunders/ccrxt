//! Example demonstrating Coinbase Exchange public REST API endpoints
//!
//! This example shows how to use the public REST endpoints for market data
//! retrieval without requiring authentication.

use reqwest::Client;

use venues::coinbase::{
    RateLimiter,
    public::rest::{
        GetProductBookRequest, GetProductCandlesRequest, GetProductRequest, GetProductStatsRequest,
        GetProductTickerRequest, GetProductTradesRequest, GetProductVolumeSummaryRequest,
        GetProductsRequest, RestClient,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the public REST client (no authentication needed)
    let client = RestClient::new(
        "https://api.exchange.coinbase.com",
        Client::new(),
        RateLimiter::new(),
    );

    println!("=== Coinbase Exchange Public REST API Example ===\n");

    // Example 1: Get all available products
    println!("1. Getting all available products...");
    let products_request = GetProductsRequest::default();

    match client.get_products(&products_request).await {
        Ok(products) => {
            println!("✓ Found {} products", products.len());

            // Show first 5 products
            for (i, product) in products.iter().take(5).enumerate() {
                println!(
                    "  {}. {} - {} (status: {})",
                    i + 1,
                    product.id,
                    product.display_name,
                    product.status
                );
            }
            if products.len() > 5 {
                println!("  ... and {} more", products.len() - 5);
            }
        }
        Err(e) => println!("✗ Error getting products: {}", e),
    }

    println!();

    // Example 2: Get volume summary for all products
    println!("2. Getting volume summary for all products...");
    let volume_request = GetProductVolumeSummaryRequest::default();

    match client.get_product_volume_summary(&volume_request).await {
        Ok(volume_summaries) => {
            println!("✓ Found {} volume summaries", volume_summaries.len());

            // Show top 3 by 24h volume
            let mut sorted_volumes = volume_summaries;
            sorted_volumes.sort_by(|a, b| {
                let a_vol: f64 = a.spot_volume_24hour.parse().unwrap_or(0.0);
                let b_vol: f64 = b.spot_volume_24hour.parse().unwrap_or(0.0);
                b_vol.partial_cmp(&a_vol).unwrap()
            });

            for (i, summary) in sorted_volumes.iter().take(3).enumerate() {
                println!(
                    "  {}. {} - 24h volume: {} {}",
                    i + 1,
                    summary.id,
                    summary.spot_volume_24hour,
                    summary.base_currency
                );
            }
        }
        Err(e) => println!("✗ Error getting volume summary: {}", e),
    }

    println!();

    // Example 3: Get specific product information (BTC-USD)
    println!("3. Getting BTC-USD product information...");
    let product_request = GetProductRequest::default();

    match client.get_product("BTC-USD", &product_request).await {
        Ok(product) => {
            println!("✓ BTC-USD Product Info:");
            println!("  Display Name: {}", product.display_name);
            println!("  Base Currency: {}", product.base_currency);
            println!("  Quote Currency: {}", product.quote_currency);
            println!("  Quote Increment: {}", product.quote_increment);
            println!("  Min Market Funds: {}", product.min_market_funds);
            println!("  Status: {}", product.status);
            println!("  Trading Disabled: {}", product.trading_disabled);
        }
        Err(e) => println!("✗ Error getting BTC-USD product: {}", e),
    }

    println!();

    // Example 4: Get order book for BTC-USD
    println!("4. Getting BTC-USD order book (Level 2)...");
    let book_request = GetProductBookRequest {
        level: Some(2), // Full aggregated order book
    };

    match client.get_product_book("BTC-USD", &book_request).await {
        Ok(order_book) => {
            println!("✓ Order Book:");
            println!("  Sequence: {}", order_book.sequence);
            println!("  Auction Mode: {}", order_book.auction_mode);
            println!("  Bids: {} levels", order_book.bids.len());
            println!("  Asks: {} levels", order_book.asks.len());

            // Show best bid and ask
            if !order_book.bids.is_empty() {
                println!("  Best Bid: {:?}", order_book.bids[0]);
            }
            if !order_book.asks.is_empty() {
                println!("  Best Ask: {:?}", order_book.asks[0]);
            }
        }
        Err(e) => println!("✗ Error getting order book: {}", e),
    }

    println!();

    // Example 5: Get 24h stats for BTC-USD
    println!("5. Getting BTC-USD 24h statistics...");
    let stats_request = GetProductStatsRequest::default();

    match client.get_product_stats("BTC-USD", &stats_request).await {
        Ok(stats) => {
            println!("✓ 24h Statistics:");
            println!("  Open: ${}", stats.open);
            println!("  High: ${}", stats.high);
            println!("  Low: ${}", stats.low);
            println!("  Last: ${}", stats.last);
            println!("  Volume: {} BTC", stats.volume);
            println!("  30-day Volume: {} BTC", stats.volume_30day);
        }
        Err(e) => println!("✗ Error getting stats: {}", e),
    }

    println!();

    // Example 6: Get current ticker for BTC-USD
    println!("6. Getting BTC-USD current ticker...");
    let ticker_request = GetProductTickerRequest::default();

    match client.get_product_ticker("BTC-USD", &ticker_request).await {
        Ok(ticker) => {
            println!("✓ Current Ticker:");
            println!("  Bid: ${}", ticker.bid);
            println!("  Ask: ${}", ticker.ask);
            println!("  Last Price: ${}", ticker.price);
            println!("  Last Size: {} BTC", ticker.size);
            println!("  24h Volume: {} BTC", ticker.volume);
            println!("  Last Trade ID: {}", ticker.trade_id);
            println!("  Time: {}", ticker.time);
        }
        Err(e) => println!("✗ Error getting ticker: {}", e),
    }

    println!();

    // Example 7: Get recent trades for BTC-USD
    println!("7. Getting recent BTC-USD trades...");
    let trades_request = GetProductTradesRequest {
        limit: Some(5),
        before: None,
        after: None,
    };

    match client.get_product_trades("BTC-USD", &trades_request).await {
        Ok((trades, pagination)) => {
            println!("✓ Recent Trades:");
            for (i, trade) in trades.iter().enumerate() {
                println!(
                    "  {}. Trade #{}: {} {} BTC @ ${} ({})",
                    i + 1,
                    trade.trade_id,
                    trade.side,
                    trade.size,
                    trade.price,
                    trade.time
                );
            }

            if let Some(pagination) = pagination {
                println!(
                    "  Pagination: before={:?}, after={:?}",
                    pagination.before, pagination.after
                );
            }
        }
        Err(e) => println!("✗ Error getting trades: {}", e),
    }

    println!();

    // Example 8: Get historical candles for BTC-USD
    println!("8. Getting BTC-USD historical candles (1 hour)...");
    let candles_request = GetProductCandlesRequest {
        granularity: Some(3600), // 1 hour candles
        start: None,
        end: None,
    };

    match client
        .get_product_candles("BTC-USD", &candles_request)
        .await
    {
        Ok(candles) => {
            println!("✓ Historical Candles:");
            println!("  Found {} candles", candles.len());

            // Show first 3 candles
            for (i, candle) in candles.iter().take(3).enumerate() {
                println!(
                    "  {}. Timestamp: {}, OHLC: ${}/{}/{}/{}, Volume: {}",
                    i + 1,
                    candle.timestamp(),
                    candle.open(),
                    candle.high(),
                    candle.low(),
                    candle.close(),
                    candle.volume()
                );
            }

            if candles.len() > 3 {
                println!("  ... and {} more candles", candles.len() - 3);
            }
        }
        Err(e) => println!("✗ Error getting candles: {}", e),
    }

    println!("\n=== Example completed ===");
    println!("All public endpoints successfully demonstrated!");

    Ok(())
}
