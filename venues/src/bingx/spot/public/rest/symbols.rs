use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult, SymbolStatus};

const SYMBOLS_ENDPOINT: &str = "/openApi/spot/v1/common/symbols";

/// Request for the spot trading symbols endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetSymbolsRequest {
    /// Trading pair, e.g., BTC-USDT (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Timestamp of initiating the request, Unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
    /// Request valid time window value, Unit: milliseconds (required)
    pub timestamp: i64,
}

/// Response from the spot trading symbols endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetSymbolsResponse {
    /// List of symbols
    pub symbols: Vec<Symbol>,
}

/// Symbol information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// Trading pair
    pub symbol: String,
    /// Price step
    pub tick_size: f64,
    /// Quantity step
    pub step_size: f64,
    /// Minimum quantity (deprecated, use min_notional/price formula)
    pub min_qty: f64,
    /// Maximum quantity (deprecated, use max_notional/price formula)
    pub max_qty: f64,
    /// Minimum transaction amount
    pub min_notional: f64,
    /// Maximum transaction amount
    pub max_notional: f64,
    /// Status: 0 offline, 1 online, 5 pre-open, 25 trading suspended
    pub status: SymbolStatus,
    /// Available buy via api
    pub api_state_buy: bool,
    /// Available sell via api
    pub api_state_sell: bool,
    /// Online time
    pub time_online: i64,
    /// Offline time
    pub off_time: i64,
    /// Trading suspension time
    pub maintain_time: i64,
}

impl RestClient {
    /// Get spot trading symbols
    ///
    /// Get information about trading pairs, including price/quantity steps,
    /// minimum/maximum transaction amounts, and trading status.
    ///
    /// # Arguments
    /// * `request` - The symbols request parameters
    ///
    /// # Returns
    /// Response containing list of symbols with their trading parameters
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v1/common/symbols
    /// - For price reference, check GET /openApi/spot/v1/ticker/24hr
    pub async fn get_symbols(&self, request: &GetSymbolsRequest) -> RestResult<GetSymbolsResponse> {
        self.send_request(SYMBOLS_ENDPOINT, Some(request), EndpointType::PublicMarket)
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;
    use crate::bingx::spot::RateLimiter;

    #[test]
    fn test_symbols_request_creation() {
        let timestamp = 1640995200000;
        let request = GetSymbolsRequest {
            symbol: None,
            recv_window: None,
            timestamp,
        };

        assert_eq!(request.timestamp, timestamp);
        assert!(request.symbol.is_none());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_symbols_request_for_symbol() {
        let timestamp = 1640995200000;
        let symbol = "BTC-USDT".to_string();
        let request = GetSymbolsRequest {
            symbol: Some(symbol.clone()),
            recv_window: None,
            timestamp,
        };

        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.symbol, Some(symbol));
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_symbols_request_with_recv_window() {
        let timestamp = 1640995200000;
        let recv_window = 5000;
        let request = GetSymbolsRequest {
            symbol: None,
            recv_window: Some(recv_window),
            timestamp,
        };

        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.recv_window, Some(recv_window));
    }

    #[test]
    fn test_symbols_request_serialization() {
        let request = GetSymbolsRequest {
            symbol: None,
            recv_window: None,
            timestamp: 1640995200000,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[tokio::test]
    async fn test_get_symbols_method_exists() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetSymbolsRequest {
            symbol: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_symbols(&request).await.is_err());
    }
}
