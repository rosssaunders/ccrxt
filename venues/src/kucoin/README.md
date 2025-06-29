# KuCoin Venue Implementation

This module provides a Rust implementation for interacting with the KuCoin exchange API.

## Documentation

- **Source Documentation**: [KuCoin API Documentation](https://docs.kucoin.com/)
- **Authentication Type**: API Key + Secret + Passphrase (HMAC-SHA256)
- **API Version**: v1 and v2

## Authentication

KuCoin requires three components for authentication:

- **API Key**: Your API key from KuCoin
- **API Secret**: Your secret key for signing requests
- **API Passphrase**: Your passphrase (encrypted with the secret)

Authentication headers required:

- `KC-API-KEY`: Your API key
- `KC-API-SIGN`: HMAC-SHA256 signature
- `KC-API-TIMESTAMP`: Unix timestamp in milliseconds
- `KC-API-PASSPHRASE`: HMAC-SHA256 encrypted passphrase
- `KC-API-KEY-VERSION`: Version 2

## Implemented Endpoints

### Public REST API

- ✅ **Server Time** - `GET /api/v1/timestamp`
- ✅ **Currencies** - `GET /api/v1/currencies` and `GET /api/v1/currencies/{currency}`
- ✅ **Symbols** - `GET /api/v1/symbols`
- ✅ **Ticker Statistics** - `GET /api/v1/market/orderbook/level1` and `GET /api/v1/market/allTickers`
- ✅ **Order Book** - `GET /api/v1/market/orderbook/level2_{20|100}` and `GET /api/v1/market/orderbook/level2`
- ✅ **Trade History** - `GET /api/v1/market/histories`
- ✅ **Klines/Candlesticks** - `GET /api/v1/market/candles`

### Private REST API

- ✅ **Place Order** - `POST /api/v1/hf/orders`
- ✅ **Cancel Order** - `DELETE /api/v1/hf/orders`
- ✅ **Cancel All Orders** - `DELETE /api/v1/hf/orders` (with filters)

## Usage Examples

### Public API

```rust
use venues::kucoin::public::{RestClient, GetServerTimeRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RestClient::new_default();
    
    // Get server time
    let (response, _) = client.get_server_time(GetServerTimeRequest::default()).await?;
    println!("Server time: {}", response.timestamp);
    
    Ok(())
}
```

### Private API

```rust
use venues::kucoin::private::{RestClient, PlaceOrderRequest};
use venues::kucoin::{OrderSide, OrderType};
use rest::secrets::SecretValue;
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = SecretValue::new(SecretString::new("your_api_key".to_string()));
    let api_secret = SecretValue::new(SecretString::new("your_api_secret".to_string()));
    let passphrase = SecretValue::new(SecretString::new("your_passphrase".to_string()));
    
    let client = RestClient::new_with_credentials(api_key, api_secret, passphrase);
    
    let request = PlaceOrderRequest {
        side: OrderSide::Buy,
        symbol: "BTC-USDT".to_string(),
        order_type: OrderType::Limit,
        price: Some("50000.0".to_string()),
        size: Some("0.001".to_string()),
        ..Default::default()
    };
    
    let (response, _) = client.place_order(request).await?;
    println!("Order placed: {}", response.order_id);
    
    Ok(())
}
```

## Rate Limiting

KuCoin has various rate limits:

- Public endpoints: Generally more lenient
- Private endpoints: More restrictive

The rate limiter is built into the client and will prevent requests when limits are approached.

## Error Handling

All methods return a `Result<T, KucoinError>` where `KucoinError` includes:

- HTTP errors
- API errors with specific error codes
- Authentication failures
- Rate limit violations
- JSON parsing errors

## Sandbox Environment

For testing, you can use the sandbox environment:

```rust
let client = RestClient::new_sandbox(api_key, api_secret, passphrase);
```

## Feature Completeness

This implementation covers the most commonly used endpoints for spot trading. Additional endpoints can be added as needed following the same patterns established in this codebase.
