# Binance Options Rust Example CLI

This example CLI demonstrates how to interact with Binance Options endpoints using the Rust `venues` library. It showcases the trading rules implementation including PRICE_FILTER and LOT_SIZE filters.

## Features

- **Trading Rules**: Demonstrates PRICE_FILTER and LOT_SIZE filter handling
- **Exchange Information**: Fetches option symbols and their trading rules
- **Filter Validation**: Shows how disabled filter rules are handled (when values are "0")

## Prerequisites

- Rust 1.70 or later
- Internet connection for API calls

## Usage

All commands are run via the `binanceoption` binary. You can use the `--testnet` flag to run against the Binance testnet instead of mainnet.

### Get Exchange Information

Fetches current exchange trading rules, symbol information, and rate limits. This is a public endpoint and does not require API credentials.

```sh
cargo run --bin binanceoption -- exchange-info
```

**Testnet:**
```sh
cargo run --bin binanceoption -- --testnet exchange-info
```

### Example Output

```
Exchange timezone: UTC
Number of rate limits: 3
Number of option symbols: 125

Sample Option Symbol Trading Rules:
Symbol: BTC-240329-73000-C
Underlying: BTC
Option Type: Call
Strike Price: 73000
Status: Trading

Trading Filters:
  PRICE_FILTER:
    - Minimum price: 0.0001
    - Maximum price: 100000.0000
    - Price tick size: 0.0001
  LOT_SIZE:
    - Minimum quantity: 0.001
    - Maximum quantity: 100000.000
    - Quantity step size: 0.001

Request completed in: 234ms
```

## Help

```sh
cargo run --bin binanceoption -- --help
```

## Trading Rules Details

### PRICE_FILTER

The PRICE_FILTER defines the price rules for an option symbol:

- **minPrice**: Minimum price allowed; disabled when value is "0"
- **maxPrice**: Maximum price allowed; disabled when value is "0"  
- **tickSize**: Price intervals that can be increased/decreased; disabled when value is "0"

### LOT_SIZE

The LOT_SIZE filter defines the quantity rules for an option symbol:

- **minQty**: Minimum quantity allowed; disabled when value is "0"
- **maxQty**: Maximum quantity allowed; disabled when value is "0"
- **stepSize**: Quantity intervals that can be increased/decreased; disabled when value is "0"

## API Documentation

For more information about Binance Options API, see:
- [Binance Options Common Definitions](https://developers.binance.com/docs/derivatives/option/common-definition)