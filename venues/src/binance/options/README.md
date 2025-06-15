# Binance Options Private REST API Client

This module provides a complete implementation of the Binance Options private REST API client, following the same patterns as other exchange implementations in this codebase.

## Features

- ✅ **Private REST Client**: Authenticated requests with HMAC-SHA256 signing
- ✅ **Rate Limiting**: Built-in rate limiting following Binance Options API guidelines
- ✅ **Error Handling**: Comprehensive error handling for API responses
- ✅ **Account Management**: Get account information, assets, and Greeks
- ✅ **Order Management**: Place, cancel, and query orders
- ✅ **Position Management**: Query current positions
- ✅ **Trade History**: Retrieve trading history
- ✅ **Builder Pattern**: Fluent API for request construction

## Supported Endpoints

### Account Information
- `GET /eapi/v1/account` - Get current account information

### Order Management
- `POST /eapi/v1/order` - Place new option order
- `DELETE /eapi/v1/order` - Cancel option order

### Position Information
- `GET /eapi/v1/position` - Get current position information

### Trading History
- `GET /eapi/v1/userTrades` - Get account trading history

## Usage

```rust
use venues::binance::options::{PrivateRestClient, RateLimiter};
use venues::binance::options::private::rest::{AccountRequest, NewOrderRequest};
use reqwest::Client;

// Create the client
let api_key = Box::new(your_api_key);
let api_secret = Box::new(your_api_secret);
let base_url = "https://eapi.binance.com";
let rate_limiter = RateLimiter::new();
let http_client = Client::new();

let client = PrivateRestClient::new(
    api_key,
    api_secret,
    base_url,
    rate_limiter,
    http_client,
);

// Get account information
let account_info = client.get_account(AccountRequest::default()).await?;
println!("Account: {:?}", account_info);

// Place a new order
let order_request = NewOrderRequest::new(
    "BTC-200730-9000-C".to_string(),
    OrderSide::Buy,
    OptionsOrderType::Limit,
    "1.0".to_string(),
)
.price("2000.0".to_string());

let order_result = client.new_order_ack(order_request).await?;
println!("Order placed: {:?}", order_result);

// Get positions
let positions = client.get_all_positions().await?;
println!("Positions: {:?}", positions);
```

## API Documentation Reference

This implementation is based on the official Binance Options API documentation:
- Base URL: `https://eapi.binance.com`
- API Prefix: `/eapi/v1/`
- Authentication: HMAC-SHA256 signing
- Rate Limiting: 6000 weight per minute, 1200 orders per minute

## Rate Limiting

The client automatically handles rate limiting with the following limits:
- **Request Weight**: 6000 per minute
- **Raw Requests**: 61000 per 5 minutes  
- **Orders**: 100 per 10 seconds, 1200 per minute

Rate limiting headers are automatically parsed and used to update internal counters.

## Error Handling

The client provides comprehensive error handling for:
- Network errors
- API errors (with error codes)
- Rate limit violations
- Authentication failures
- Malformed requests/responses

## Security

- API credentials are handled using the `ExposableSecret` trait for secure storage
- HMAC-SHA256 request signing
- Automatic timestamp generation
- Secure header handling