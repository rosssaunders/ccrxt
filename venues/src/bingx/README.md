# BingX Integration

This module provides a comprehensive Rust client for the BingX exchange API, supporting both public market data and private account endpoints.

## Links

- **Exchange Website**: [BingX](https://bingx.com)
- **API Documentation**: [BingX API Docs](https://bingx-api.github.io/docs/)
- **Broker Program**: [BingX Broker Application](https://docs.google.com/forms/d/e/1FAIpQLSfO4Ws3UO13h_9tcnRKKGJD6QTvTM8q32hmpNQlzB4tofup7g/viewform)

## Authentication

BingX uses **API Key + Secret** authentication for private endpoints. Public endpoints do not require authentication.

### API Key Setup

1. [Create Account](https://bingx.com)
2. [Pass KYC/KYB](https://bingx.com/en-us/account/api/)
3. [Create API KEY](https://bingx.com/en-us/account/api/)
4. [Configure API KEY permissions](https://bingx.com/en-us/account/api/)

Each parent user can create up to 20 API keys, and each sub-user can also create up to 20 API keys with different permissions.

## Public API Endpoints Implemented

### Server Information
- ✅ **Server Time** - Get server timestamp

### Market Data
- ✅ **Spot Trading Symbols** - Get all trading pairs and their metadata
- ✅ **Recent Trades List** - Get recent trades for a symbol
- ✅ **Order Book** - Get current order book depth
- ✅ **Kline/Candlestick Data** - Get current kline data with multiple intervals
- ✅ **24hr Ticker Statistics** - Get 24-hour price change statistics
- ✅ **Order Book Aggregation** - Get aggregated order book with different precision levels
- ✅ **Symbol Price Ticker** - Get latest price for a symbol
- ✅ **Symbol Order Book Ticker** - Get best bid/ask prices
- ✅ **Historical K-line** - Get historical kline data
- ✅ **Old Trade Lookup** - Get historical trade data

### Supported Features

#### Time Intervals
The following kline intervals are supported:
- `1m`, `3m`, `5m`, `15m`, `30m`
- `1h`, `2h`, `4h`, `6h`, `8h`, `12h`
- `1d`, `3d`, `1w`, `1M`

#### Order Book Precision
Multiple aggregation levels are available:
- `step0` - Default precision
- `step1` to `step5` - 10x to 100,000x aggregated precision

#### Symbol Status
- **Online** - Active trading
- **Offline** - Not available for trading
- **Pre-open** - Preparing for trading
- **Trading Suspended** - Temporarily suspended

## Rate Limits

BingX implements several rate limiting strategies:

### IP Rate Limits
- **Market API Group [1]**: 100 requests per 10 seconds total for all interfaces
- **Account API Group [2]**: 1000 requests per 10 seconds total, 100 per interface
- **Account API Group [3]**: 1000 requests per 10 seconds total, 200 per interface

### Request Headers
Monitor rate limit usage via HTTP headers:
- `X-RateLimit-Requests-Remain` - Remaining requests in current window
- `X-RateLimit-Requests-Expire` - Window expiration timestamp

## Error Handling

The client handles common BingX error codes:

- `100001` - Signature verification failed
- `100202` - Insufficient balance
- `100204` - No data
- `100400` - Invalid parameter
- `100440` - Order price deviates greatly from market price
- `100500` - Server error
- `100503` - Server busy

## Usage Examples

### Basic Market Data

```rust
use reqwest::Client;
use venues::bingx::{
    public::PublicRestClient, GetSymbolsRequest, GetKlineRequest, 
    Interval, RateLimiter,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PublicRestClient::new(
        "https://open-api.bingx.com",
        Client::new(),
        RateLimiter::new(),
    );

    // Get all symbols
    let symbols_request = GetSymbolsRequest::new(
        chrono::Utc::now().timestamp_millis()
    );
    let symbols_response = client.get_symbols(&symbols_request).await?;
    
    // Get kline data
    let kline_request = GetKlineRequest::new(
        "BTC-USDT".to_string(),
        Interval::OneHour,
        chrono::Utc::now().timestamp_millis(),
    );
    let kline_response = client.get_kline(&kline_request).await?;

    Ok(())
}
```

### Order Book Data

```rust
use venues::bingx::{GetOrderBookRequest, GetOrderBookAggregationRequest, DepthType};

// Get standard order book
let order_book_request = GetOrderBookRequest::new(
    "BTC-USDT".to_string(),
    chrono::Utc::now().timestamp_millis(),
).with_limit(20);

let order_book = client.get_order_book(&order_book_request).await?;

// Get aggregated order book
let aggregation_request = GetOrderBookAggregationRequest::new(
    "BTC_USDT".to_string(),
    50,
    DepthType::Step0,
);

let aggregated_book = client.get_order_book_aggregation(&aggregation_request).await?;
```

## Examples

See the `examples/bingx/` directory for comprehensive usage examples:

- `public_rest_api_example.rs` - Complete overview of all public endpoints
- `symbols_example.rs` - Symbol information and filtering
- `market_data_example.rs` - Market data analysis and price tracking

## Private API

Private API endpoints for account management, trading, and portfolio operations are implemented in the `private` module. These require API key authentication.

## Network Configuration

### Primary Endpoint
- **Main**: `https://open-api.bingx.com`

### Backup Endpoint
- **Alternate**: `https://open-api.bingx.io` (60 requests/minute limit)
- Use only when primary endpoint is unavailable

## Support

- **API Issues**: [Telegram Group](https://t.me/+uSWmuaKA5sw2MzE1)
- **User Survey**: [Feedback Form](https://docs.google.com/forms/d/e/1FAIpQLSd0yjx5okwQG1D7tf4pBAcf4WbMW8zE-Ew01ardWGCwoIZoMg/viewform)
