use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const TRADING_PAIRS_LIST_ENDPOINT: &str = "/spot/v1/symbols";

/// Request parameters for getting trading pairs list
#[derive(Debug, Serialize, Default)]
pub struct GetTradingPairsListRequest {
    // No parameters needed for this endpoint
}

/// Response for trading pairs list endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradingPairsListResponse {
    /// Array of trading pairs
    pub symbols: Vec<String>,
}

impl RestClient {
    /// Get Trading Pairs List (V1)
    ///
    /// Get a list of all trading pairs on the platform
    ///
    /// [docs](https://developer-pro.bitmart.com/en/spot/#get-trading-pairs-list-v1)
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Returns
    /// List of all trading pairs on the platform
    ///
    /// # Note
    /// - Returns an array of trading pairs
    /// - "BMX_ETH" means that the base currency of this trading pair is BMX, and the quote currency is ETH
    pub async fn get_trading_pairs_list(
        &self,
        _request: GetTradingPairsListRequest,
    ) -> RestResult<GetTradingPairsListResponse> {
        self.send_get_request(
            TRADING_PAIRS_LIST_ENDPOINT,
            Option::<&()>::None, // No query parameters
            EndpointType::SpotPublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trading_pairs_list_request_default() {
        let request = GetTradingPairsListRequest::default();
        // Request has no fields to check
        let _ = request;
    }

    #[test]
    fn test_get_trading_pairs_list_response_structure() {
        let response = GetTradingPairsListResponse {
            symbols: vec![
                "BMX_ETH".to_string(),
                "XLM_ETH".to_string(),
                "MOBI_ETH".to_string(),
            ],
        };

        assert_eq!(response.symbols.len(), 3);
        assert_eq!(response.symbols[0], "BMX_ETH");
        assert_eq!(response.symbols[1], "XLM_ETH");
        assert_eq!(response.symbols[2], "MOBI_ETH");
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "symbols": [
                "BMX_ETH",
                "XLM_ETH",
                "MOBI_ETH"
            ]
        }"#;

        let response: GetTradingPairsListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbols.len(), 3);
        assert_eq!(response.symbols[0], "BMX_ETH");
        assert_eq!(response.symbols[1], "XLM_ETH");
        assert_eq!(response.symbols[2], "MOBI_ETH");
    }

    #[test]
    fn test_empty_response() {
        let json = r#"{
            "symbols": []
        }"#;

        let response: GetTradingPairsListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbols.len(), 0);
    }

    #[test]
    fn test_trading_pairs_serialization_roundtrip() {
        let response = GetTradingPairsListResponse {
            symbols: vec!["BTC_USDT".to_string(), "ETH_USDT".to_string()],
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetTradingPairsListResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.symbols.len(), deserialized.symbols.len());
        assert_eq!(response.symbols[0], deserialized.symbols[0]);
        assert_eq!(response.symbols[1], deserialized.symbols[1]);
    }
}
