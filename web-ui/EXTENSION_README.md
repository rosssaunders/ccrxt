# CCRXT API Explorer Chrome Extension

A Chrome extension that provides a web-based interface for testing cryptocurrency exchange APIs with secure credential storage and real API calls.

## Features

- 🔐 **Secure Credential Storage**: API keys are stored using Chrome's secure storage API
- 🌐 **CORS Bypass**: Direct API calls to exchanges without CORS restrictions
- 📊 **505 API Endpoints**: Auto-generated from CCRXT library with full documentation
- 🔄 **Real-time Testing**: Execute live API calls and see responses instantly
- 📋 **Comprehensive Documentation**: Each endpoint includes parameters, authentication requirements, and descriptions

## Supported Exchanges

- Coinbase Advanced Trade
- Binance (Spot, USDM Futures, COINM Futures)
- Deribit
- OKX
- Crypto.com
- Bitmart
- BingX

## Installation

### Option 1: Development Mode

1. **Build the extension**:
   ```bash
   cd web-ui
   trunk build --release
   cp manifest.json background.js dist/
   ```

2. **Load in Chrome**:
   - Open Chrome and go to `chrome://extensions/`
   - Enable "Developer mode" in the top right
   - Click "Load unpacked"
   - Select the `dist/` folder

### Option 2: Production Package

1. **Create extension package**:
   ```bash
   cd web-ui
   trunk build --release
   cp manifest.json background.js dist/
   cd dist && zip -r ../ccrxt-extension.zip . && cd ..
   ```

2. **Install from package**:
   - Go to `chrome://extensions/`
   - Enable "Developer mode"
   - Click "Load unpacked" and select the `dist/` folder

## Usage

### 1. Configure API Credentials

1. Click the extension icon in Chrome toolbar
2. Go to the "Credentials" tab
3. Select your exchange
4. Enter your API credentials:
   - **API Key**: Your exchange API key
   - **API Secret**: Your exchange API secret
   - **Passphrase**: Required for some exchanges (e.g., Coinbase)
5. Click "Save Credentials"

**Security Note**: Credentials are stored locally in Chrome's secure storage and never leave your browser.

### 2. Test API Endpoints

1. Go to the "Endpoints" tab
2. Select an exchange from the dropdown
3. Choose a category (Market Data, Trading, Account, etc.)
4. Select a specific endpoint
5. Fill in required parameters
6. Click "Execute" to make the API call

### 3. View Results

- **Success**: JSON response from the exchange API
- **Error**: Detailed error message with debugging information
- **Authentication**: Endpoints requiring credentials will show a warning

## API Authentication

The extension handles authentication automatically for supported exchanges:

- **Coinbase**: Uses CB-ACCESS-* headers with HMAC-SHA256 signatures
- **Binance**: Uses X-MBX-APIKEY header with query parameter signatures
- **Other exchanges**: Authentication methods vary by exchange

## Security Features

- ✅ Credentials stored in Chrome's secure local storage
- ✅ No credentials sent to external servers
- ✅ All API calls made directly from your browser to exchanges
- ✅ Extension permissions limited to specific exchange domains
- ✅ HTTPS-only connections to exchange APIs

## Troubleshooting

### Extension Not Working

1. Check that developer mode is enabled in `chrome://extensions/`
2. Verify the extension is loaded and active
3. Check browser console for error messages

### API Calls Failing

1. Verify your API credentials are correct
2. Check that your IP is whitelisted on the exchange (if required)
3. Ensure API keys have the necessary permissions
4. Some exchanges require additional verification steps

### Rate Limiting

- The extension respects exchange rate limits
- If you exceed limits, wait before making more requests
- Consider using lower-frequency calls for testing

## Development

To modify the extension:

1. **Edit source code** in `src/` directory
2. **Rebuild**: `trunk build --release`
3. **Copy files**: `cp manifest.json background.js dist/`
4. **Reload extension** in Chrome extensions page

### File Structure

```
dist/
├── manifest.json          # Extension manifest
├── background.js         # Service worker for API calls
├── index.html           # Main popup HTML
├── ccrxt-web-ui-*.js    # Compiled WASM/JS bundle
└── ccrxt-web-ui-*_bg.wasm # WebAssembly binary
```

## Privacy Policy

This extension:
- ✅ Does NOT collect any personal data
- ✅ Does NOT send data to external servers
- ✅ Only communicates with cryptocurrency exchanges you explicitly configure
- ✅ Stores credentials locally in your browser only

## Support

For issues or questions:
1. Check the browser console for error messages
2. Verify your API credentials and permissions
3. Ensure you're using the latest version of the extension
4. File issues on the project repository

## License

This project is part of the CCRXT library ecosystem.