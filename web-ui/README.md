# CCRXT Web UI

A Leptos-based web application for testing and exploring CCRXT cryptocurrency exchange API endpoints.

## Features

### 🔑 API Credential Management
- Secure storage of API keys and secrets in browser local storage
- Support for multiple exchanges with different authentication requirements
- Automatic detection of passphrase requirements (e.g., Coinbase)

### 🏪 Multi-Exchange Support
- **Coinbase**: Full public and private REST API support
- **Binance Spot**: Market data and trading endpoints
- **Deribit**: Futures and options trading
- **Easy extension**: Framework ready for additional exchanges

### 📊 Dynamic API Explorer
- **Endpoint Discovery**: Browse endpoints by exchange and category
- **Parameter Forms**: Auto-generated forms based on API specifications
- **Type Safety**: Parameter validation with proper types (strings, numbers, enums, etc.)
- **Real-time Validation**: Immediate feedback on required fields and constraints

### 🛡️ Security & UX
- **Authentication Indicators**: Visual cues for endpoints requiring API credentials
- **Mock API Calls**: Safe testing environment with simulated responses
- **Error Handling**: Comprehensive validation with user-friendly error messages
- **Responsive Design**: Clean, modern interface that works on desktop and mobile

## Quick Start

### Prerequisites
- Rust (latest stable)
- `trunk` for building WebAssembly applications

### Installation

1. **Install Trunk** (if not already installed):
   ```bash
   cargo install --locked trunk
   ```

2. **Build the application**:
   ```bash
   ./build.sh
   ```

3. **Serve locally** (development):
   ```bash
   trunk serve
   ```
   The app will be available at `http://127.0.0.1:8080`

4. **Production build**:
   ```bash
   trunk build --release
   ```

## Usage

### 1. Configure API Credentials
1. Select your exchange from the dropdown
2. Enter your API key and secret
3. For exchanges like Coinbase, add your passphrase
4. Click "Save Credentials" - they're stored securely in your browser

### 2. Explore API Endpoints
1. Choose an exchange and category (Market Data, Trading, Account, etc.)
2. Select an endpoint from the dropdown
3. Fill in the required parameters using the auto-generated form
4. Click "Execute API Call" to test the endpoint

### 3. View Results
- See formatted JSON responses
- Review parameter validation errors
- Understand endpoint requirements and authentication needs

## Supported Endpoints

### Coinbase
- **Public**: `get_products`, `get_product_book`, `get_product_ticker`, `get_product_candles`
- **Private**: `create_order`, `get_account_balances`, `get_orders`

### Binance Spot  
- **Public**: `get_exchange_info`, `get_depth`, `get_ticker_24hr`
- **Private**: `new_order`, `get_account`

### Deribit
- **Public**: `get_instruments`, `get_order_book`, `ticker`
- **Private**: `get_open_orders_by_currency`, `get_open_orders_by_instrument`

## Architecture

### Component Structure
```
src/
├── app.rs                 # Main application component
├── components/
│   ├── credentials.rs     # API credential management
│   ├── endpoints.rs       # Endpoint selection and execution
│   └── parameter_form.rs  # Dynamic parameter input forms
├── endpoints.rs           # API endpoint definitions
├── types.rs              # Core type definitions
└── utils.rs              # Utility functions
```

### Key Features

**Dynamic Form Generation**: Parameter forms are automatically generated from endpoint specifications, including:
- Type-specific input fields (text, number, select, etc.)
- Required field validation
- Enum dropdowns with predefined values
- Min/max constraints for numeric inputs

**Type Safety**: Full Rust type safety throughout the application:
- Compile-time verification of endpoint structures
- Runtime parameter validation
- Strongly-typed venue and endpoint categories

**Extensible Design**: Easy to add new exchanges:
1. Add venue to `Venue` enum
2. Create endpoint definitions in `endpoints.rs`
3. No UI changes required - forms auto-generate

## Development

### Adding New Endpoints
1. **Define the endpoint** in `src/endpoints.rs`:
   ```rust
   ApiEndpoint::new(
       "unique_id".to_string(),
       Venue::YourExchange,
       "endpoint_name".to_string(),
       HttpMethod::GET,
       "/api/path".to_string(),
       EndpointCategory::MarketData,
       AuthType::None,
       "Description".to_string(),
   )
   .with_parameter(Parameter {
       name: "symbol".to_string(),
       param_type: ParameterType::String,
       required: true,
       // ... other fields
   })
   ```

2. **Add to venue endpoints** in the appropriate `create_*_endpoints()` function

3. **The UI automatically updates** with the new endpoint

### Adding New Exchanges
1. Add to `Venue` enum in `src/endpoints.rs`
2. Implement credential requirements in `src/types.rs`  
3. Create endpoint definitions following existing patterns
4. UI components automatically support the new exchange

## Technical Details

- **Framework**: Leptos 0.6 with Client-Side Rendering (CSR)
- **Styling**: Custom CSS with modern, clean design
- **Storage**: Browser LocalStorage for credential management
- **Build**: Trunk + WebAssembly for efficient web deployment
- **Type System**: Comprehensive parameter validation with Rust's type safety

## Security

- **Local Storage**: Credentials never leave your browser
- **Mock Mode**: All API calls are currently mocked for safety
- **No Network**: Current version doesn't make real API calls (safe for testing)

## Future Enhancements

- [ ] Real API call integration with CCRXT venue clients
- [ ] WebSocket support for real-time data
- [ ] Response history and comparison
- [ ] API rate limit monitoring
- [ ] Export/import credential configurations
- [ ] Dark mode support

## Contributing

This web UI is part of the CCRXT project. To contribute:

1. Fork the CCRXT repository
2. Make changes in the `web-ui/` directory
3. Ensure all tests pass: `cargo test`
4. Submit a pull request

## License

MIT License - same as the parent CCRXT project.