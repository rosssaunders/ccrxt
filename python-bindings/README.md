# CCRXT Python Bindings

Automatic PyO3 bindings for the CCRXT cryptocurrency exchange API library.

## 🎯 Naming Convention-Based Generation

This system automatically generates Python bindings for your Rust code based on **naming conventions** - no manual annotation required!

### Automatic Detection Rules

The build system automatically exposes:

#### Structs
- `*Request` - API request structures
- `*Response` - API response structures  
- `*Client` - API client structures
- `*Error` - Error types (with special handling)
- `*Info`, `*Data`, `*Config` - Configuration/data structures
- `*Order`, `*Trade`, `*Account`, `*Balance` - Trading structures
- `*Ticker`, `*Kline`, `*Depth` - Market data structures
- `*Position`, `*Symbol`, `*Filter` - Trading/symbol structures

#### Enums
- All public enums (except `*Error` enums)

#### Implementations
- `*Client` implementations
- `*Request` implementations  
- `*Builder` implementations

### Generated Python API

```python
import ccrxt

# Binance Spot API
client = ccrxt.binance.spot.RestClient()
await client.get_exchange_info()

# Kucoin API
kucoin_client = ccrxt.kucoin.spot.RestClient()
await kucoin_client.get_ticker("BTC-USDT")

# Request objects
request = ccrxt.binance.spot.ExchangeInfoRequest()
request.symbol = "BTCUSDT"
```

## 🚀 Quick Start

### 1. Build the Bindings

```bash
# Automated setup
./tools/setup_python_bindings.sh

# OR manual setup
cd python-bindings
pip install maturin
maturin develop --release
```

### 2. Test the Bindings

```bash
python3 tools/test_bindings.py
```

### 3. Use in Python

```python
import asyncio
import ccrxt

async def main():
    # Create a client (automatically detected from naming)
    client = ccrxt.binance.spot.PublicRestClient()
    
    # Make API calls (async methods automatically wrapped)
    exchange_info = await client.get_exchange_info()
    print(f"Exchange: {exchange_info.timezone}")
    
    # Use request objects (automatically detected)
    ticker_request = ccrxt.binance.spot.TickerRequest()
    ticker_request.symbol = "BTCUSDT"
    ticker = await client.get_ticker(ticker_request)

asyncio.run(main())
```

## 🔧 How It Works

### Build-Time Generation

1. **AST Analysis**: The build script scans all Rust files in `venues/src/`
2. **Naming Convention Detection**: Identifies structs, enums, and impls to expose
3. **PyO3 Code Generation**: Generates Python bindings automatically
4. **Module Organization**: Creates organized Python modules per venue

### Key Features

- **Zero Manual Annotation**: No need to add `#[pyclass]` to every struct
- **Automatic Async Handling**: Async Rust methods become Python awaitable
- **Type Conversion**: Automatic conversion between Rust and Python types
- **Error Handling**: Rust `Result` types become Python exceptions
- **Module Structure**: Preserves your Rust module hierarchy in Python

### Generated Files

```
python-bindings/src/generated/
├── mod.rs              # Main module
├── binance.rs          # Binance venue bindings
├── kucoin.rs           # Kucoin venue bindings
├── okx.rs              # OKX venue bindings
└── ...                 # Other venues
```

## 📚 Examples

### Basic Trading

```python
import ccrxt
import asyncio

async def place_order():
    client = ccrxt.binance.spot.PrivateRestClient()
    
    # Create order request (detected by naming)
    order_request = ccrxt.binance.spot.OrderRequest()
    order_request.symbol = "BTCUSDT"
    order_request.side = ccrxt.binance.spot.OrderSide.BUY
    order_request.order_type = ccrxt.binance.spot.OrderType.LIMIT
    order_request.quantity = 0.001
    order_request.price = 50000.0
    
    # Place order (async method automatically wrapped)
    order = await client.place_order(order_request)
    print(f"Order placed: {order.order_id}")
```

### Market Data

```python
import ccrxt
import asyncio

async def get_market_data():
    client = ccrxt.binance.spot.PublicRestClient()
    
    # Get ticker (method detected and wrapped)
    ticker = await client.get_ticker("BTCUSDT")
    print(f"Price: {ticker.price}")
    
    # Get order book
    depth = await client.get_depth("BTCUSDT", 10)
    print(f"Best bid: {depth.bids[0].price}")
```

## 🛠️ Development

### Adding New Venues

1. Create your Rust venue in `venues/src/your_venue/`
2. Follow the naming conventions for structs and methods
3. Run `cargo build` - bindings are generated automatically!

### Customizing Detection

Edit `build.rs` to modify the naming convention patterns:

```rust
fn should_expose_struct(s: &ItemStruct) -> bool {
    let name = s.ident.to_string();
    let expose_patterns = [
        "Request", "Response", "Client", "YourCustomPattern"
    ];
    expose_patterns.iter().any(|pattern| name.contains(pattern))
}
```

### Type Stubs

Generate Python type stubs for better IDE support:

```bash
cd python-bindings
maturin develop
stubgen -m ccrxt -o python/ccrxt/stubs/
```

## 📈 Benefits

1. **Automatic**: No manual PyO3 attributes needed
2. **Consistent**: Same naming conventions across all venues
3. **Maintainable**: Changes to Rust code automatically reflect in Python
4. **Scalable**: Works with hundreds of structs and methods
5. **Type-Safe**: Preserves Rust's type safety in Python

## 🎛️ Configuration

### Build Configuration

Modify `python-bindings/Cargo.toml`:

```toml
[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
venues = { path = "../venues" }
# ... other dependencies
```

### Python Configuration

Modify `python-bindings/pyproject.toml`:

```toml
[project]
name = "ccrxt"
version = "0.1.0"
# ... other settings
```

## 🚢 Distribution

Build Python wheels:

```bash
cd python-bindings
maturin build --release
```

Upload to PyPI:

```bash
maturin publish
```

## 📄 License

MIT License - see LICENSE file for details.
