# Bullish Exchange Integration

This directory contains the Rust implementation for the Bullish Exchange API integration.

## Overview

Bullish is a digital asset exchange that offers spot trading, derivatives, and various financial products. This implementation provides comprehensive access to both public and private API endpoints.

## Source Documentation

- [Bullish Trading API Documentation](https://api.exchange.bullish.com/docs/)
- [Bullish Support](https://support.bullish.com)
- [API Error & Rejection Codes](https://github.com/bullish-exchange/api-docs/wiki/Error-&-Rejection-Codes)
- [Institutional User Guide](https://support.bullish.com/hc/en-us/articles/28811587741721-Comprehensive-New-User-Guide-for-Institutions)

## Authentication

Bullish uses **JWT Bearer Token + HMAC Signature** authentication for private endpoints:

- API Key and Secret are required
- JWT tokens are generated using HMAC-SHA256 signing
- Private endpoints require both JWT authorization and HMAC signature headers
- Tokens automatically refresh when expired

## Rate Limiting

The API enforces rate limits across different endpoint categories:
- **Public endpoints**: 50 requests/second
- **Private endpoints**: 50 requests/second (with higher limits available via rate limit tokens)
- **Login endpoints**: 10 requests/second
- **Time endpoints**: 100 requests/second

## Implemented Endpoints

### Public REST API Endpoints

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/v1/markets` | GET | Get all markets | ✅ |
| `/v1/markets/{symbol}` | GET | Get specific market | ✅ |
| `/v1/markets/{symbol}/orderbook/hybrid` | GET | Get orderbook | ✅ |
| `/v1/markets/{symbol}/trades` | GET | Get public trades | ✅ |
| `/v1/markets/{symbol}/tick` | GET | Get 24h ticker | ✅ |
| `/v1/markets/{symbol}/candles` | GET | Get candlestick data | ✅ |
| `/v1/assets` | GET | Get all assets | ✅ |
| `/v1/assets/{symbol}` | GET | Get specific asset | ✅ |
| `/v1/index-prices` | GET | Get all index prices | ✅ |
| `/v1/index-prices/{assetSymbol}` | GET | Get index price by symbol | ✅ |
| `/v1/nonce` | GET | Get current nonce range | ✅ |
| `/v1/time` | GET | Get server time | ✅ |

### Private REST API Endpoints

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/v1/accounts/trading-accounts` | GET | Get trading accounts | ✅ |
| `/v1/accounts/trading-accounts/{id}` | GET | Get specific trading account | ✅ |
| `/v1/accounts/asset` | GET | Get asset balances | ✅ |
| `/v1/accounts/asset/{symbol}` | GET | Get specific asset balance | ✅ |
| `/v2/orders` | GET | Get orders | ✅ |
| `/v2/orders` | POST | Create order | ✅ |
| `/v2/orders/{orderId}` | GET | Get specific order | ✅ |
| `/v1/trades` | GET | Get trades | ✅ |
| `/v1/trades/{tradeId}` | GET | Get specific trade | ✅ |
| `/v1/wallets/transactions` | GET | Get wallet transactions | ✅ |

### Planned Endpoints (Future Implementation)

- Order amendment (`POST /v2/command#amend`)
- Order cancellation
- Wallet deposit/withdrawal instructions
- Derivatives positions
- Historical data endpoints
- AMM instructions
- Portfolio margin simulation

## Usage Examples

### Public API Usage

```rust
use venues::bullish::public::RestClient;
use venues::bullish::RateLimiter;

let client = RestClient::new(
    "https://api.exchange.bullish.com",
    reqwest::Client::new(),
    RateLimiter::new(),
);

// Get all markets
let markets = client.get_markets().await?;

// Get order book
let orderbook = client.get_orderbook("BTCUSDC", None).await?;

// Get ticker
let ticker = client.get_ticker("BTCUSDC").await?;
```

### Private API Usage

```rust
use venues::bullish::private::RestClient;
use venues::bullish::{CreateOrderRequest, OrderSide, OrderType, TimeInForce};
use rest::secrets::SecretString;

let client = RestClient::new(
    Box::new(SecretString::new("your_api_key".to_string())),
    Box::new(SecretString::new("your_api_secret".to_string())),
    "https://api.exchange.bullish.com",
    reqwest::Client::new(),
    RateLimiter::new(),
);

// Get trading accounts
let accounts = client.get_trading_accounts().await?;

// Place an order
let order_request = CreateOrderRequest {
    command_type: "V3CreateOrder".to_string(),
    client_order_id: "my_order_123".to_string(),
    symbol: "BTCUSDC".to_string(),
    order_type: OrderType::Limit,
    side: OrderSide::Buy,
    price: Some("30000.00".to_string()),
    quantity: "0.001".to_string(),
    time_in_force: TimeInForce::Gtc,
    allow_borrow: false,
    trading_account_id: "your_trading_account_id".to_string(),
    ..Default::default()
};

let response = client.create_order(order_request).await?;
```

## Error Handling

The implementation provides comprehensive error handling:

```rust
match client.create_order(request).await {
    Ok(response) => {
        println!("Order placed: {}", response.order_id);
    }
    Err(venues::bullish::Errors::AuthenticationError(msg)) => {
        eprintln!("Authentication failed: {}", msg);
    }
    Err(venues::bullish::Errors::RateLimitError(msg)) => {
        eprintln!("Rate limit exceeded: {}", msg);
    }
    Err(venues::bullish::Errors::ApiError(api_err)) => {
        eprintln!("API error {}: {}", api_err.code, api_err.message);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Environment Configuration

For production use, consider these environments:

1. **Production**: `https://api.exchange.bullish.com`
2. **Sandbox**: Contact Bullish for sandbox environment details

## Connectivity Options

Bullish offers three connectivity options:

1. **Cloudflare** (Default): Standard internet connection
2. **Cloudflare Bypass**: Optimized connection with improved latency
3. **Direct Connect**: AWS/GCP private connection for institutional clients

For Cloudflare Bypass or Direct Connect, contact your Bullish sales representative.

## Security Considerations

- ⚠️ Never hardcode API credentials in your code
- Use environment variables or secure credential management
- Validate all input parameters before sending requests
- Implement proper error handling and logging
- Consider using the rate limit token for higher throughput

## Testing

Run the examples:

```bash
# Public API examples
cargo run --example bullish_public_markets    # Market data and assets
cargo run --example bullish_market_data       # Comprehensive market data
cargo run --example bullish_nonce_example     # Get nonce range
cargo run --example bullish_index_prices_example  # Index prices
cargo run --example bullish_candles_example   # Candlestick data

# Private trading example (requires credentials)
cargo run --example bullish_private_trading
```

## Support

- Check the [Bullish Status Page](https://bullish.statuspage.io/) for operational status
- Review [Error & Rejection Codes](https://github.com/bullish-exchange/api-docs/wiki/Error-&-Rejection-Codes) for troubleshooting
- Contact Bullish support for API-related issues
