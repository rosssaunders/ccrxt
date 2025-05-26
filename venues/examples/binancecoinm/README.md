# Binance COIN-M Rust Example CLI

This example CLI demonstrates how to interact with Binance COIN-M endpoints using the Rust `venues` library. It supports account info, trade history, and batch order placement.

## Prerequisites

- Rust (https://rustup.rs/)
- Binance COIN-M API credentials

## Setup

1. **Clone the repository and navigate to this directory:**

```sh
cd venues/examples/binancecoinm
```

2. **Set up your environment variables:**

Create a `.env` file in this directory with your Binance API credentials:

```env
API_KEY=your_api_key_here
API_SECRET=your_api_secret_here
```

> **Note:** Your API key and secret must have the required permissions for the endpoints you wish to use.

3. **Build the CLI:**

```sh
cargo build --bin binancecoinm
```

## Usage

All commands are run via the `binancecoinm` binary. You can use the `--testnet` flag to run against the Binance testnet instead of mainnet.

### Get Account Information

Fetches your account balances and margin info.

```sh
cargo run --bin binancecoinm -- account
```

**Testnet:**
```sh
cargo run --bin binancecoinm -- --testnet account
```

### Get Recent Trades

Fetches recent trades for a given symbol (e.g., BTCUSD_PERP).

```sh
cargo run --bin binancecoinm -- trades BTCUSD_PERP
```

**With a custom limit (e.g., 50):**
```sh
cargo run --bin binancecoinm -- trades BTCUSD_PERP --limit 50
```

**Testnet:**
```sh
cargo run --bin binancecoinm -- --testnet trades BTCUSD_PERP
```

### Place a Batch Order

Places a single order (or batch, if extended) for a symbol. For LIMIT orders, the `--price` argument is required.

```sh
cargo run --bin binancecoinm -- batch-order BTCUSD BUY LIMIT 0.01 --price 60000
```

- `<SYMBOL>`: Trading pair symbol (e.g., BTCUSD)
- `<SIDE>`: BUY or SELL
- `<ORDER_TYPE>`: LIMIT or MARKET
- `<QUANTITY>`: Order quantity (e.g., 0.01)
- `--price <PRICE>`: Required for LIMIT orders

**Market order example:**
```sh
cargo run --bin binancecoinm -- batch-order BTCUSD BUY MARKET 0.01
```

**Testnet:**
```sh
cargo run --bin binancecoinm -- --testnet batch-order BTCUSD_PERP SELL LIMIT 1 --price 100000
```

## Help

To see all available commands and options:

```sh
cargo run --bin binancecoinm -- --help
```

Or for a specific command:

```sh
cargo run --bin binancecoinm -- batch-order --help
```

## Notes
- All output is printed to the console.
- Errors are reported with descriptive messages.
- Ensure your API key/secret are correct and have the necessary permissions.
- Use the `--testnet` flag for safe testing without real funds.
