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

- API Key and Secret are required (stored as SecretString)
- Obtain a JWT via HMAC login (`GET /trading-api/v1/users/hmac/login`) or session login (`POST /trading-api/v2/users/login`)
- Private GET endpoints require the `Authorization: Bearer <token>` header
- Private POST/PUT/DELETE endpoints also require HMAC headers: `BX-TIMESTAMP`, `BX-NONCE`, `BX-SIGNATURE`
- When a token expires, calls will fail with Unauthorized; acquire a new token via `hmac_login` or `login` and retry

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
| `/trading-api/v1/markets` | GET | Get all markets | ✅ |
| `/trading-api/v1/markets/{symbol}` | GET | Get specific market | ✅ |
| `/trading-api/v1/markets/{symbol}/orderbook/hybrid` | GET | Get orderbook | ✅ |
| `/trading-api/v1/markets/{symbol}/trades` | GET | Get public trades | ✅ |
| `/trading-api/v1/markets/{symbol}/tick` | GET | Get 24h ticker | ✅ |
| `/trading-api/v1/markets/{symbol}/candle` | GET | Get candlestick data | ✅ |
| `/trading-api/v1/assets` | GET | Get all assets | ✅ |
| `/trading-api/v1/assets/{symbol}` | GET | Get specific asset | ✅ |
| `/trading-api/v1/index-prices` | GET | Get all index prices | ✅ |
| `/trading-api/v1/index-prices/{assetSymbol}` | GET | Get index price by symbol | ✅ |
| `/trading-api/v1/nonce` | GET | Get current nonce range | ✅ |
| `/trading-api/v1/time` | GET | Get server time | ✅ |

### Private REST API Endpoints

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/trading-api/v1/accounts/trading-accounts` | GET | Get trading accounts | ✅ |
| `/trading-api/v1/accounts/trading-accounts/{id}` | GET | Get specific trading account | ✅ |
| `/trading-api/v1/accounts/asset` | GET | Get asset balances | ✅ |
| `/trading-api/v1/accounts/asset/{symbol}` | GET | Get specific asset balance | ✅ |
| `/trading-api/v2/orders` | GET | Get orders | ✅ |
| `/trading-api/v2/orders` | POST | Create order | ✅ |
| `/trading-api/v2/orders/{orderId}` | GET | Get specific order | ✅ |
| `/trading-api/v1/trades` | GET | Get trades | ✅ |
| `/trading-api/v1/trades/{tradeId}` | GET | Get specific trade | ✅ |
| `/trading-api/v2/history/orders` | GET | Get historical orders | ✅ |
| `/trading-api/v1/history/trades` | GET | Get historical trades | ✅ |
| `/trading-api/v1/history/derivatives-settlement` | GET | Get historical derivatives settlement | ✅ |
| `/trading-api/v1/wallets/transactions` | GET | Get wallet transactions (token pagination) | ✅ |
| `/trading-api/v1/wallets/limits` | GET | Get custody withdrawal limits | ✅ |
| `/trading-api/v1/wallets/deposit-instructions/crypto` | GET | Get crypto deposit instructions | ✅ |
| `/trading-api/v1/wallets/deposit-instructions/fiat` | GET | Get fiat deposit instructions | ✅ |
| `/trading-api/v1/wallets/withdrawal-instructions/crypto` | GET | Get crypto withdrawal instructions | ✅ |
| `/trading-api/v1/derivatives-positions` | GET | Get derivatives positions | ✅ |
| `/trading-api/v2/amm-instructions` | POST | Create AMM instruction | ✅ |
| `/trading-api/v2/amm-instructions` | GET | Get AMM instructions | ✅ |
| `/trading-api/v2/amm-instructions/{instructionId}` | GET | Get specific AMM instruction | ✅ |
| `/trading-api/v2/command` | POST | Amend order | ✅ |
| `/trading-api/v2/command` | POST | Cancel order | ✅ |
| `/trading-api/v2/command` | POST | Cancel all orders | ✅ |
| `/trading-api/v2/command` | POST | Cancel all orders by market | ✅ |
| `/trading-api/v2/command` | POST | Delayed cancel all orders | ✅ |
| `/trading-api/v2/command` | POST | Unset delayed cancel all orders | ✅ |
| `/trading-api/v1/simulate-portfolio-margin` | POST | Portfolio margin simulator | ✅ |
| `/trading-api/v1/users/hmac/login` | GET | HMAC login (get JWT) | ✅ |
| `/trading-api/v2/users/login` | POST | Session login (get JWT) | ✅ |
| `/trading-api/v1/users/logout` | POST | Logout | ✅ |
| `/trading-api/v1/command?commandType=V1TransferAsset` | POST | Transfer asset between trading accounts | ✅ |

### Planned Endpoints (Future Implementation)

- Additional private endpoints as Bullish expands API coverage
- Streaming (WebSocket) market data (venue-agnostic infra exists in repo)

## Usage Examples

### Public API Usage

```rust
use venues::bullish::{PublicRestClient, RateLimiter, PaginatedResult, PaginationParams};
use venues::bullish::public::rest::{GetTickerRequest, OrderbookRequest, GetCandlesRequest};
use venues::bullish::CandleInterval;

let client = PublicRestClient::new(
    "https://api.exchange.bullish.com",
    reqwest::Client::new(),
    RateLimiter::new(),
);

// Get all markets
let markets = client.get_markets().await?;

// Get order book
let orderbook = client.get_orderbook("BTCUSDC", None).await?;

// Get ticker
let ticker = client.get_ticker(&GetTickerRequest { symbol: "BTCUSDC".into() }).await?;

// Get candles with pagination helpers
let candles_req = GetCandlesRequest {
    symbol: "BTCUSDC".into(),
    interval: Some(CandleInterval::OneMinute),
    created_at_datetime_gte: None,
    created_at_datetime_lte: None,
    pagination: PaginationParams { page_size: Some(25), meta_data: Some(true), next_page: None, previous_page: None },
};
let candles = client.get_market_candle(&candles_req).await?;
let data = match candles {
    PaginatedResult::Direct(d) => d,
    PaginatedResult::Paginated { data, .. } => data,
    PaginatedResult::Token { data, .. } => data,
};
```

### Private API Usage

```rust
use venues::bullish::{PrivateRestClient, Credentials, RateLimiter};
use venues::bullish::{CreateOrderRequest};
use venues::bullish::enums::{OrderSide, OrderType, TimeInForce};
use venues::bullish::private::rest::create_order::CommandType;
use rest::secrets::SecretString;

let creds = Credentials {
    api_key: SecretString::new("your_api_key".to_string().into_boxed_str()),
    api_secret: SecretString::new("your_api_secret".to_string().into_boxed_str()),
};

let mut client = PrivateRestClient::new(
    creds,
    "https://api.exchange.bullish.com",
    reqwest::Client::new(),
    RateLimiter::new(),
);

// Acquire JWT (required before calling private endpoints)
let _session = client.hmac_login().await?; // or client.login(...).await?

// Get trading accounts
let accounts = client.get_trading_accounts().await?;

// Place an order
let order_request = CreateOrderRequest {
    command_type: CommandType::V3CreateOrder,
    client_order_id: "my_order_123".to_string(),
    symbol: "BTCUSDC".to_string(),
    order_type: OrderType::Limit,
    side: OrderSide::Buy,
    price: Some("30000.00".to_string()),
    quantity: "0.001".to_string(),
    quote_amount: None,
    time_in_force: TimeInForce::Gtc,
    allow_borrow: false,
    trading_account_id: "your_trading_account_id".to_string(),
};
let order_resp = client.create_order(order_request).await?;
```

#### Credentials struct

- `Credentials { api_key: SecretString, api_secret: SecretString }`
- Provide credentials securely (environment variables or secret manager); never hard-code in code

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

Run Bullish public integration tests:

```bash
cargo test --package ccrxt --test mod -- bullish::public_integration_tests --show-output
```

## Support

- Check the [Bullish Status Page](https://bullish.statuspage.io/) for operational status
- Review [Error & Rejection Codes](https://github.com/bullish-exchange/api-docs/wiki/Error-&-Rejection-Codes) for troubleshooting
- Contact Bullish support for API-related issues
