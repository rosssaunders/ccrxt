# Crypto.com Exchange Public REST API

This document provides an overview of the Crypto.com Exchange public REST API endpoints implemented in this library.

## Overview

The Crypto.com Exchange public REST API provides access to public market data including:

- Market information and instruments
- Order book data
- Recent trades
- Candlestick/OHLC data
- Ticker information
- Insurance fund information
- Risk parameters

## Base URL

The public REST API base URL is:
```
https://deriv-api.crypto.com
```

## Rate Limits

All public endpoints have rate limits to ensure fair usage:
- General endpoints: 100 requests per 10 seconds
- Market data endpoints: 100 requests per 10 seconds

## Available Endpoints

### Market Information

#### Get Instruments
- **Endpoint**: `public/get-instruments`
- **Method**: GET
- **Description**: Provides information on all available instruments

#### Get Risk Parameters  
- **Endpoint**: `public/get-risk-parameters`
- **Method**: GET
- **Description**: Get risk parameters for trading instruments

### Market Data

#### Get Order Book
- **Endpoint**: `public/get-book`
- **Method**: GET
- **Description**: Retrieves the order book for a given instrument
- **Parameters**:
  - `instrument_name` (required): e.g., "BTCUSD-PERP"
  - `depth` (optional): Number of price levels (default: 20, max: 50)

#### Get Recent Trades
- **Endpoint**: `public/get-trades`
- **Method**: GET
- **Description**: Get recent trades for a given instrument
- **Parameters**:
  - `instrument_name` (required): e.g., "BTCUSD-PERP"
  - `count` (optional): Number of trades to return (default: 100, max: 200)

#### Get Candlestick Data
- **Endpoint**: `public/get-candlestick`
- **Method**: GET
- **Description**: Get OHLC candlestick data for a given instrument
- **Parameters**:
  - `instrument_name` (required): e.g., "BTCUSD-PERP"
  - `timeframe` (required): Timeframe (1m, 5m, 15m, 30m, 1h, 4h, 6h, 12h, 1D, 7D, 14D, 1M)
  - `start_time` (optional): Start time in Unix timestamp format
  - `end_time` (optional): End time in Unix timestamp format

**Note**: For historical data requests, the maximum allowed time window is **7 days**. This applies to all time-based queries to ensure optimal performance and compliance with exchange policies.

#### Get Ticker Information
- **Endpoint**: `public/get-tickers`
- **Method**: GET
- **Description**: Get ticker information for all or specific instruments
- **Parameters**:
  - `instrument_name` (optional): Specific instrument name

### System Information

#### Get Insurance Fund
- **Endpoint**: `public/get-insurance`
- **Method**: GET
- **Description**: Get insurance fund information

#### Get Valuations
- **Endpoint**: `public/get-valuations`
- **Method**: GET
- **Description**: Get current valuations for supported instruments

#### Get Announcements
- **Endpoint**: `public/get-announcements`
- **Method**: GET
- **Description**: Get system announcements and maintenance notifications

#### Get Conversion Rate
- **Endpoint**: `public/get-conversion-rate`
- **Method**: GET
- **Description**: Get conversion rate between currencies

#### Get Expired Settlement Price
- **Endpoint**: `public/get-expired-settlement-price`
- **Method**: GET
- **Description**: Get settlement prices for expired instruments

## Error Handling

All endpoints return standardized error responses with:
- Error code
- Error message
- Additional details where applicable

Common error codes:
- `10001`: Invalid parameter
- `10002`: Required parameter missing
- `10003`: Invalid instrument name
- `10004`: Request timeout
- `10005`: Rate limit exceeded

## Implementation Notes

- All timestamps are in Unix format (milliseconds)
- All price and quantity fields are returned as strings to maintain precision
- The maximum time window for historical data queries is 7 days
- Parameters should be URL-encoded when making requests
- Responses are in JSON format

## Examples

### Get Order Book
```bash
GET /public/get-book?instrument_name=BTCUSD-PERP&depth=10
```

### Get Recent Trades
```bash
GET /public/get-trades?instrument_name=BTCUSD-PERP&count=50
```

### Get Candlestick Data
```bash
GET /public/get-candlestick?instrument_name=BTCUSD-PERP&timeframe=1h&start_time=1640995200000&end_time=1641081600000
```

**Important**: When requesting historical candlestick data, ensure the time window between `start_time` and `end_time` does not exceed 7 days (604,800,000 milliseconds).