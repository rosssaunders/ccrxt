# Gate.io Integration Tests

This directory contains comprehensive integration tests for Gate.io's REST API endpoints, organized by product type.

## Test Structure

The tests are organized following Gate.io's documentation structure:

- `spot/` - Spot trading integration tests
- `delivery/` - Delivery trading integration tests  
- `perpetual/` - Perpetual/futures trading integration tests
- `options/` - Options trading integration tests
- `unified/` - Unified trading integration tests
- `mod.rs` - Module exports

### Product Type Directories

Each product type has its own directory with:
- `mod.rs` - Module exports for the product type
- `public_integration_tests.rs` - Test suite for public endpoints

## Covered Endpoints

### Spot Trading (`spot/`)
- **Server Time** (`/spot/time`) - Validates server connectivity and time synchronization
- **Client Creation** - Tests REST client initialization and configuration

### Market Data Endpoints
- **Currency Pairs** (`/spot/currency_pairs`) - Lists all available trading pairs with trading fees and precision
- **Currencies** (`/spot/currencies`) - Lists all supported currencies with trading status
- **Tickers** (`/spot/tickers`) - 24h price statistics for all pairs or specific pairs
- **Order Book** (`/spot/order_book`) - Current bid/ask prices with configurable depth (5-100 levels)
- **Trades** (`/spot/trades`) - Recent public trades with time range filtering and pagination
- **Candlesticks** (`/spot/candlesticks`) - OHLCV data with multiple time intervals (1h, 1d)

### Advanced Features
- **Error Handling** - Tests invalid inputs and API error responses
- **Rate Limiting** - Validates built-in rate limiting functionality
- **Pagination** - Tests trade pagination with different page sizes
- **Time Filtering** - Tests historical data retrieval with time ranges
- **Multiple Intervals** - Tests candlestick data across different timeframes

## Test Features

### Rate Limiting Compliance
- Tests include delays (500ms-1000ms) between requests to respect Gate.io's rate limits
- Single-threaded execution (`--test-threads=1`) to avoid concurrent request limits
- Built-in rate limiter validation to prevent API blocking

### Data Validation
- Validates response structure and data types
- Ensures required fields are present and non-empty
- Tests array formats for candlestick data (handles 6-8 element arrays)
- Validates numeric data ranges and formats

### Real Market Data
- All tests use live Gate.io API endpoints
- Tests against real trading pairs (BTC_USDT, ETH_USDT, LTC_USDT)
- Validates actual market data responses

## Running Tests

### Local Testing

```bash
# Run all Gate.io integration tests
cargo test --test gateio_tests

# Run with single thread (recommended for rate limiting)  
cargo test --test gateio_tests -- --test-threads=1

# Run specific product type tests
cargo test gateio::spot --test gateio_tests
cargo test gateio::delivery --test gateio_tests  
cargo test gateio::perpetual --test gateio_tests
cargo test gateio::options --test gateio_tests
cargo test gateio::unified --test gateio_tests

# Run specific test
cargo test gateio::spot::public_integration_tests::test_get_server_time --test gateio_tests
```

### Automated Testing
Gate.io tests run automatically via GitHub Actions:
- **Schedule**: Nightly at 4 AM UTC
- **Workflow**: `.github/workflows/gateio-nightly-integration-tests.yml`
- **Manual Trigger**: Available via GitHub Actions UI

## Test Configuration

### Rate Limiting
Tests are configured with appropriate delays to avoid hitting Gate.io's rate limits:
- Basic requests: 500ms delay
- Complex operations: 1000ms delay
- Reduced test matrix to minimize API calls

### Timeout Settings
- Individual tests: 30 minutes maximum
- Single requests: Standard timeout (30 seconds)
- Sequential execution to avoid rate limit conflicts

## Fixed Issues

### Response Structure Fixes
1. **Ticker Fields** - Made `change_utc0` and `change_utc8` optional fields
2. **Trade Fields** - Made `role` field optional as it's not always returned
3. **Candlestick Format** - Handles flexible array formats (6-8 elements)

### Rate Limiter Improvements
- Added overflow protection for reset time calculations
- Increased delays between requests to prevent API throttling
- Implemented single-threaded test execution

## Monitoring

### Success Metrics
- All 19 test cases should pass consistently
- API response times under 5 seconds
- No rate limit violations or 429 errors

### Failure Handling
- Tests continue on error to collect maximum information
- Detailed logging for debugging API issues
- Artifact collection for post-mortem analysis

## API Coverage

The test suite covers 7 core public endpoints:
- ✅ Server time
- ✅ Currency pairs listing (300+ pairs)
- ✅ Currency information (100+ currencies)
- ✅ Market tickers (all & specific)
- ✅ Order book data (multiple depths)
- ✅ Trade history (with pagination)
- ✅ Candlestick charts (multiple intervals)

This represents comprehensive coverage of Gate.io's public market data API suitable for real-time trading applications.