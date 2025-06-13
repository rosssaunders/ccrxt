Rules

1. All venues to implement the low latency APIs. If Websocket available, use that over REST calls.
2. All venue rate limiting to be implemented exactly.
3. All wrappers around the endpoints should be pure. Not fixes and helper functions.
4. All websockets to implement the common websocket trait.

## Examples

### Binance Portfolio Margin

The `binanceportfoliomargin` example demonstrates how to create a Binance Portfolio Margin API client for authenticated requests.

```bash
# Run the Portfolio Margin example
cargo run -p binanceportfoliomargin-example

# With API credentials from environment variables
BINANCE_API_KEY=your_key BINANCE_API_SECRET=your_secret cargo run -p binanceportfoliomargin-example
```

This example shows:
- Creating a Portfolio Margin private REST client
- Using the https://papi.binance.com base URL
- Proper credential handling with encryption
- Rate limiting and error handling

### Coinbase Market Data

The `coinbase_market_data` example demonstrates how to subscribe to various market data streams from Coinbase Advanced Trade. You can customize which products and data types you want to receive.

```bash
# Subscribe to all channels for BTC-USD only
cargo run --example coinbase_market_data -- --products "BTC-USD"

# Subscribe only to orderbook and ticker for multiple products
cargo run --example coinbase_market_data -- --products "BTC-USD,ETH-USD" --trades false --heartbeats false

# Subscribe to all channels for multiple products
cargo run --example coinbase_market_data -- --products "BTC-USD,ETH-USD,SOL-USD"
```

Available options:
- `-p, --products`: Comma-separated list of products to subscribe to (e.g., "BTC-USD,ETH-USD,SOL-USD")
- `-b, --orderbook`: Subscribe to orderbook updates (default: true)
- `-i, --ticker`: Subscribe to ticker updates (default: true)
- `-t, --trades`: Subscribe to market trades (default: true)
- `-h, --heartbeats`: Subscribe to heartbeats (default: true)