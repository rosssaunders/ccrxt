# CCRXT Python Bindings - Implementation Guide

This document provides a comprehensive guide to the automatic PyO3 binding generation system for the CCRXT cryptocurrency exchange API library.

## 🔧 System Architecture

### Core Components

1. **Build Script** (`python-bindings/build.rs`)
   - Scans `venues/src/` for Rust source files
   - Applies naming convention detection
   - Generates PyO3 bindings automatically
   - Creates organized Python modules per venue

2. **Naming Convention Engine**
   - Pattern-based struct/enum/impl detection
   - Automatic exposure decisions
   - Consistent API surface generation

3. **Code Generation Pipeline**
   - AST parsing with `syn`
   - Code generation with `quote`
   - PyO3 wrapper creation
   - Module organization

## 🎯 Naming Convention Rules

### Automatically Exposed Structs

The system exposes structs with these naming patterns:

```rust
// API structures
pub struct ExchangeInfoRequest { ... }      // *Request
pub struct ExchangeInfoResponse { ... }     // *Response
pub struct BinanceSpotClient { ... }        // *Client

// Data structures
pub struct OrderInfo { ... }                // *Info
pub struct TradeData { ... }                // *Data
pub struct SystemConfig { ... }             // *Config

// Trading structures  
pub struct SpotOrder { ... }                // *Order
pub struct Trade { ... }                    // *Trade
pub struct Account { ... }                  // *Account
pub struct Balance { ... }                  // *Balance
pub struct Position { ... }                 // *Position

// Market data structures
pub struct Ticker { ... }                   // *Ticker
pub struct Kline { ... }                    // *Kline
pub struct Depth { ... }                    // *Depth
pub struct Symbol { ... }                   // *Symbol
pub struct Filter { ... }                   // *Filter

// System structures
pub struct RateLimit { ... }                // *RateLimit
pub struct Status { ... }                   // *Status
pub struct Params { ... }                   // *Params
pub struct Result { ... }                   // *Result
pub struct Entry { ... }                    // *Entry
pub struct History { ... }                  // *History
pub struct Stats { ... }                    // *Stats
```

### Automatically Exposed Enums

```rust
// All public enums (except *Error enums)
pub enum OrderSide { Buy, Sell }
pub enum OrderType { Market, Limit }
pub enum TimeInForce { GTC, IOC, FOK }
```

### Automatically Exposed Implementations

```rust
// Client implementations
impl BinanceSpotClient { ... }              // *Client
impl KucoinSpotClient { ... }               // *Client

// Request builders
impl ExchangeInfoRequest { ... }            // *Request
impl OrderRequestBuilder { ... }            // *Builder
```

## 🚀 Usage Examples

### Basic Client Usage

```python
import asyncio
import ccrxt

async def main():
    # Public API client
    client = ccrxt.binance.spot.PublicRestClient()
    
    # Get exchange information
    exchange_info = await client.get_exchange_info()
    print(f"Exchange timezone: {exchange_info.timezone}")
    
    # Get ticker
    ticker = await client.get_ticker("BTCUSDT")
    print(f"Price: {ticker.price}")
    
    # Get order book
    depth = await client.get_depth("BTCUSDT", 10)
    print(f"Best bid: {depth.bids[0].price}")
    print(f"Best ask: {depth.asks[0].price}")

asyncio.run(main())
```

### Request Object Usage

```python
import asyncio
import ccrxt

async def main():
    # Create request objects
    ticker_request = ccrxt.binance.spot.TickerRequest()
    ticker_request.symbol = "BTCUSDT"
    
    depth_request = ccrxt.binance.spot.DepthRequest()
    depth_request.symbol = "BTCUSDT"
    depth_request.limit = 100
    
    # Use with client
    client = ccrxt.binance.spot.PublicRestClient()
    
    ticker = await client.get_ticker(ticker_request)
    depth = await client.get_depth(depth_request)
    
    print(f"Ticker: {ticker.price}")
    print(f"Depth: {len(depth.bids)} bids, {len(depth.asks)} asks")

asyncio.run(main())
```

### Multi-Venue Usage

```python
import asyncio
import ccrxt

async def compare_prices():
    # Create clients for different venues
    binance_client = ccrxt.binance.spot.PublicRestClient()
    kucoin_client = ccrxt.kucoin.spot.PublicRestClient()
    okx_client = ccrxt.okx.spot.PublicRestClient()
    
    # Get BTC price from all venues
    binance_ticker = await binance_client.get_ticker("BTCUSDT")
    kucoin_ticker = await kucoin_client.get_ticker("BTC-USDT")
    okx_ticker = await okx_client.get_ticker("BTC-USDT")
    
    print(f"Binance BTC price: {binance_ticker.price}")
    print(f"Kucoin BTC price: {kucoin_ticker.price}")
    print(f"OKX BTC price: {okx_ticker.price}")

asyncio.run(compare_prices())
```

### Error Handling

```python
import asyncio
import ccrxt

async def handle_errors():
    client = ccrxt.binance.spot.PublicRestClient()
    
    try:
        # This might fail
        ticker = await client.get_ticker("INVALID_SYMBOL")
    except ccrxt.binance.spot.BinanceError as e:
        print(f"Binance error: {e}")
    except Exception as e:
        print(f"Other error: {e}")

asyncio.run(handle_errors())
```

## 🔨 Development Workflow

### 1. Setting Up Development Environment

```bash
# Clone the repository
git clone <repository-url>
cd ccrxt

# Set up Python bindings
cd python-bindings
pip install maturin

# Build the bindings
maturin develop --release
```

### 2. Adding New Venues

When adding a new venue, follow these steps:

1. **Create Venue Directory**
   ```bash
   mkdir venues/src/your_venue
   ```

2. **Follow Naming Conventions**
   ```rust
   // venues/src/your_venue/mod.rs
   pub mod rest;
   pub mod types;
   
   // venues/src/your_venue/rest.rs
   pub struct YourVenueClient {
       // Client implementation
   }
   
   pub struct TickerRequest {
       pub symbol: String,
   }
   
   pub struct TickerResponse {
       pub price: f64,
       pub volume: f64,
   }
   
   impl YourVenueClient {
       pub async fn get_ticker(&self, request: TickerRequest) -> Result<TickerResponse, YourVenueError> {
           // Implementation
       }
   }
   ```

3. **Build and Test**
   ```bash
   cd python-bindings
   cargo build --release
   maturin develop --release
   ```

4. **Test in Python**
   ```python
   import ccrxt
   
   client = ccrxt.your_venue.YourVenueClient()
   ticker = await client.get_ticker(ccrxt.your_venue.TickerRequest(symbol="BTCUSDT"))
   ```

### 3. Customizing Naming Patterns

Edit `python-bindings/build.rs` to modify detection patterns:

```rust
fn should_expose_struct(s: &ItemStruct) -> bool {
    let name = s.ident.to_string();
    let expose_patterns = [
        "Request", "Response", "Client", "Info", "Data", "Config",
        "Order", "Trade", "Account", "Balance", "Position",
        "Ticker", "Kline", "Depth", "Symbol", "Filter",
        "RateLimit", "Status", "Params", "Result", "Entry",
        "History", "Stats",
        // Add your custom patterns here
        "YourCustomPattern",
    ];
    expose_patterns.iter().any(|pattern| name.contains(pattern))
}
```

### 4. Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration

# Test Python bindings
cd python-bindings
python3 ../tools/test_bindings.py
```

## 📁 Project Structure

```
ccrxt/
├── python-bindings/           # PyO3 bindings workspace
│   ├── Cargo.toml            # Rust dependencies
│   ├── pyproject.toml        # Python packaging
│   ├── build.rs              # Build-time code generation
│   ├── src/
│   │   ├── lib.rs           # Python module entry point
│   │   └── generated/       # Auto-generated bindings
│   └── README.md            # Python bindings documentation
├── venues/                   # Rust venue implementations
│   ├── src/
│   │   ├── binance/         # Binance implementation
│   │   ├── kucoin/          # Kucoin implementation
│   │   ├── okx/             # OKX implementation
│   │   └── ...              # Other venues
│   └── examples/            # Venue examples
├── tools/                    # Development tools
│   ├── setup_python_bindings.sh
│   ├── test_bindings.py
│   └── generate_build_rs.py
└── tests/                    # Integration tests
    ├── binance_tests.rs
    ├── kucoin_tests.rs
    └── ...
```

## 🎛️ Configuration

### Build Configuration

`python-bindings/Cargo.toml`:
```toml
[package]
name = "ccrxt-python"
version = "0.1.0"
edition = "2021"

[lib]
name = "ccrxt"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
venues = { path = "../venues" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### Python Configuration

`python-bindings/pyproject.toml`:
```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "ccrxt"
version = "0.1.0"
description = "Python bindings for CCRXT cryptocurrency exchange API library"
requires-python = ">=3.8"
classifiers = [
    "Programming Language :: Python :: 3",
    "Programming Language :: Rust",
    "License :: OSI Approved :: MIT License",
    "Operating System :: OS Independent",
]

[tool.maturin]
python-source = "python"
```

## 🚀 Distribution

### Building Wheels

```bash
cd python-bindings

# Build for current platform
maturin build --release

# Build for multiple platforms (if configured)
maturin build --release --target x86_64-apple-darwin
maturin build --release --target aarch64-apple-darwin
maturin build --release --target x86_64-unknown-linux-gnu
```

### Publishing to PyPI

```bash
# Test upload
maturin publish --repository testpypi

# Production upload
maturin publish
```

## 🔍 Troubleshooting

### Common Issues

1. **Build Errors**
   ```bash
   # Clean and rebuild
   cd python-bindings
   cargo clean
   maturin develop --release
   ```

2. **Import Errors**
   ```python
   # Check if module is installed
   import ccrxt
   print(dir(ccrxt))
   ```

3. **Type Errors**
   ```bash
   # Generate type stubs
   stubgen -m ccrxt -o stubs/
   ```

### Debug Build

```bash
# Debug build for better error messages
cd python-bindings
maturin develop --debug
```

## 📚 Additional Resources

- [PyO3 Documentation](https://pyo3.rs/)
- [Maturin Documentation](https://github.com/PyO3/maturin)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Python Asyncio Documentation](https://docs.python.org/3/library/asyncio.html)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add your venue/feature following naming conventions
4. Build and test the Python bindings
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details.
