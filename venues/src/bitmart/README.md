# BitMart Exchange API Implementation

This module provides support for the BitMart crypto exchange API.

## Features

- Request signing according to BitMart's requirements
- Support for private REST API with secure encrypted storage of API keys

## Authentication

BitMart API requires the following headers for authenticated requests:

1. `X-BM-KEY`: Your API key
2. `X-BM-SIGN`: Signature created using HMAC-SHA256 and Base64 encoding
3. `X-BM-TIMESTAMP`: Current timestamp in milliseconds

## Signature Generation

Signatures are generated according to BitMart's documentation:

1. Create message string: timestamp + API-Key + request_path + [query_string or request_body]
2. Sign using HMAC-SHA256 with API secret as key
3. Encode result using Base64

## Usage

```rust
use venues::bitmart::private_rest::{BitMartPrivateRest, BitMartPrivateRestError};
use venues::common::encryption;
use chrono::Utc;

// Create encryption key (in real app, this should be securely stored)
let encryption_key = [1u8; 32];

// Encrypt API credentials (store these encrypted values)
let api_key = "your_api_key";
let api_secret = "your_api_secret";
let encrypted_api_key = encryption::encrypt(api_key, &encryption_key).unwrap();
let encrypted_api_secret = encryption::encrypt(api_secret, &encryption_key).unwrap();

// Create BitMart client
let client = BitMartPrivateRest::new(
    encrypted_api_key,
    encrypted_api_secret,
    "https://api-cloud.bitmart.com".to_string(),
    encryption_key
);

// Generate timestamp
let timestamp = Utc::now().timestamp_millis().to_string();

// Sign a request
let request_path = "/spot/v1/ticker";
let request_body = "symbol=BTC_USDT";
let signature = client.sign_request(&timestamp, request_path, request_body).unwrap();

// Now you can include these in your request headers:
// X-BM-KEY: your_api_key
// X-BM-SIGN: signature
// X-BM-TIMESTAMP: timestamp
```