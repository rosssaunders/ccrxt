// Chrome Extension Background Script for CCRXT API Explorer
// Handles real API calls to cryptocurrency exchanges

// Message listener for API calls from the popup
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  console.log('Background script received message:', request);
  
  if (request.type === 'apiCall') {
    handleApiCall(request)
      .then(response => sendResponse({ success: true, data: response }))
      .catch(error => sendResponse({ success: false, error: error.message }));
    return true; // Keep the message channel open for async response
  }
  
  if (request.type === 'credentials') {
    handleCredentials(request)
      .then(response => sendResponse(response))
      .catch(error => sendResponse({ success: false, error: error.message }));
    return true; // Keep the message channel open for async response
  }
  
  sendResponse({ success: false, error: 'Unknown message type' });
});

// Handle credentials operations
async function handleCredentials(request) {
  const { venue, action, credentials } = request;
  
  switch (action) {
    case 'get':
      const storedCredentials = await getCredentials(venue);
      return { success: true, credentials: storedCredentials };
      
    case 'set':
      await storeCredentials(venue, credentials);
      return { success: true };
      
    case 'delete':
      await deleteCredentials(venue);
      return { success: true };
      
    default:
      throw new Error(`Unknown credentials action: ${action}`);
  }
}

// Handle API calls to cryptocurrency exchanges
async function handleApiCall({ venue, endpoint, method, path, parameters, pathParameters, authType }) {
  try {
    // Get API base URL for the venue
    const baseUrl = getVenueBaseUrl(venue);
    
    // Build the full URL with path parameters
    let fullPath = path;
    if (pathParameters) {
      for (const [key, value] of Object.entries(pathParameters)) {
        fullPath = fullPath.replace(`{${key}}`, encodeURIComponent(value));
      }
    }
    
    const url = new URL(fullPath, baseUrl);
    
    // Add query parameters for GET requests
    if (method === 'GET' && parameters) {
      for (const [key, value] of Object.entries(parameters)) {
        if (value !== null && value !== undefined && value !== '') {
          url.searchParams.append(key, value);
        }
      }
    }
    
    // Prepare request options
    const requestOptions = {
      method: method,
      headers: {
        'Content-Type': 'application/json',
        'User-Agent': 'CCRXT-Extension/1.0.0'
      }
    };
    
    // Add authentication if required
    if (authType !== 'none') {
      const credentials = await getCredentials(venue);
      if (!credentials) {
        throw new Error(`Authentication required for ${venue}. Please configure your API credentials.`);
      }
      
      await addAuthHeaders(requestOptions, venue, credentials, method, fullPath, parameters);
    }
    
    // Add body for POST/PUT requests
    if ((method === 'POST' || method === 'PUT') && parameters) {
      requestOptions.body = JSON.stringify(parameters);
    }
    
    console.log(`Making ${method} request to: ${url.toString()}`);
    
    // Make the API call
    const response = await fetch(url.toString(), requestOptions);
    
    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`HTTP ${response.status}: ${errorText}`);
    }
    
    const data = await response.json();
    
    return {
      status: 'success',
      data: data,
      headers: Object.fromEntries(response.headers.entries()),
      url: url.toString(),
      timestamp: new Date().toISOString()
    };
    
  } catch (error) {
    console.error('API call failed:', error);
    throw error;
  }
}

// Get base URL for each venue
function getVenueBaseUrl(venue) {
  const urls = {
    'Coinbase': 'https://api.exchange.coinbase.com',
    'BinanceSpot': 'https://api.binance.com',
    'BinanceUsdm': 'https://fapi.binance.com',
    'BinanceCoinm': 'https://dapi.binance.com',
    'Deribit': 'https://www.deribit.com/api/v2',
    'OKX': 'https://www.okx.com',
    'CryptoCom': 'https://api.crypto.com/v2',
    'Bitmart': 'https://api-cloud.bitmart.com',
    'BingX': 'https://open-api.bingx.com',
    'Bullish': 'https://api.bullish.com',
    'Bitget': 'https://api.bitget.com',
    'Bybit': 'https://api.bybit.com'
  };
  
  return urls[venue] || 'https://api.example.com';
}

// Add authentication headers based on venue requirements
async function addAuthHeaders(requestOptions, venue, credentials, method, path, parameters) {
  const timestamp = Date.now().toString();
  
  switch (venue) {
    case 'Coinbase':
      if (credentials.apiKey && credentials.apiSecret && credentials.passphrase) {
        const body = requestOptions.body || '';
        const message = timestamp + method + path + body;
        const signature = await hmacSha256(credentials.apiSecret, message);
        
        requestOptions.headers['CB-ACCESS-KEY'] = credentials.apiKey;
        requestOptions.headers['CB-ACCESS-SIGN'] = signature;
        requestOptions.headers['CB-ACCESS-TIMESTAMP'] = timestamp;
        requestOptions.headers['CB-ACCESS-PASSPHRASE'] = credentials.passphrase;
      }
      break;
      
    case 'BinanceSpot':
    case 'BinanceUsdm':
    case 'BinanceCoinm':
      if (credentials.apiKey && credentials.apiSecret) {
        requestOptions.headers['X-MBX-APIKEY'] = credentials.apiKey;
        
        // Add signature for private endpoints
        if (parameters) {
          const queryString = new URLSearchParams(parameters).toString() + `&timestamp=${timestamp}`;
          const signature = await hmacSha256(credentials.apiSecret, queryString);
          requestOptions.headers['signature'] = signature;
        }
      }
      break;
      
    case 'Deribit':
      if (credentials.apiKey && credentials.apiSecret) {
        // Deribit uses signature in the request body for JSON-RPC
        // This is a simplified implementation - full Deribit auth is more complex
        requestOptions.headers['Authorization'] = `Bearer ${credentials.apiKey}`;
      }
      break;
      
    // Add other venues as needed
    default:
      console.warn(`Authentication not implemented for ${venue}`);
  }
}

// HMAC SHA256 signature generation
async function hmacSha256(secret, message) {
  const encoder = new TextEncoder();
  const keyData = encoder.encode(secret);
  const messageData = encoder.encode(message);
  
  const cryptoKey = await crypto.subtle.importKey(
    'raw',
    keyData,
    { name: 'HMAC', hash: 'SHA-256' },
    false,
    ['sign']
  );
  
  const signature = await crypto.subtle.sign('HMAC', cryptoKey, messageData);
  return btoa(String.fromCharCode(...new Uint8Array(signature)));
}

// Store credentials securely
async function storeCredentials(venue, credentials) {
  const key = `credentials_${venue}`;
  await chrome.storage.local.set({ [key]: credentials });
}

// Retrieve credentials
async function getCredentials(venue) {
  const key = `credentials_${venue}`;
  const result = await chrome.storage.local.get([key]);
  return result[key] || null;
}

// Delete credentials
async function deleteCredentials(venue) {
  const key = `credentials_${venue}`;
  await chrome.storage.local.remove([key]);
}

// Handle extension icon click - open in new tab
chrome.action.onClicked.addListener((tab) => {
  chrome.tabs.create({
    url: chrome.runtime.getURL('index.html')
  });
});

// Extension installation/update handler
chrome.runtime.onInstalled.addListener((details) => {
  if (details.reason === 'install') {
    console.log('CCRXT API Explorer installed');
  } else if (details.reason === 'update') {
    console.log('CCRXT API Explorer updated');
  }
});