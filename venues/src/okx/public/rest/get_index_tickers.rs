use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for getting index tickers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexTickersRequest {
    /// Quote currency filter (USD/USDT/BTC/USDC)
    #[serde(rename = "quoteCcy", skip_serializing_if = "Option::is_none")]
    pub quote_ccy: Option<String>,
    /// Index ID filter (e.g., "BTC-USD")
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Individual index ticker details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexTicker {
    /// Index ID (e.g., "BTC-USD")
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Latest index price
    #[serde(rename = "idxPx")]
    pub idx_px: String,
    /// Highest price in the past 24 hours
    #[serde(rename = "high24h")]
    pub high_24h: String,
    /// Lowest price in the past 24 hours
    #[serde(rename = "low24h")]
    pub low_24h: String,
    /// Open price in the past 24 hours
    #[serde(rename = "open24h")]
    pub open_24h: String,
    /// Open price in the UTC 0
    #[serde(rename = "sodUtc0")]
    pub sod_utc0: String,
    /// Open price in the UTC 8
    #[serde(rename = "sodUtc8")]
    pub sod_utc8: String,
    /// Index price update time, Unix timestamp format in milliseconds
    pub ts: String,
}

/// Response for getting index tickers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIndexTickersResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Index ticker data
    pub data: Vec<IndexTicker>,
}

impl RestClient {
    /// Get index tickers
    ///
    /// Retrieve index tickers.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-market-data-get-index-tickers
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The index tickers request parameters
    ///
    /// # Returns
    /// Response containing the list of index tickers
    pub async fn get_index_tickers(
        &self,
        request: Option<GetIndexTickersRequest>,
    ) -> RestResult<GetIndexTickersResponse> {
        self.send_request(
            "api/v5/market/index-tickers",
            reqwest::Method::GET,
            request.as_ref(),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_index_tickers_request_structure() {
        let request = GetIndexTickersRequest {
            quote_ccy: Some("USD".to_string()),
            inst_id: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("quoteCcy").and_then(|v| v.as_str()),
            Some("USD")
        );
        assert!(serialized.get("instId").is_none());
    }

    #[test]
    fn test_get_index_tickers_request_with_inst_id() {
        let request = GetIndexTickersRequest {
            quote_ccy: None,
            inst_id: Some("BTC-USD".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert!(serialized.get("quoteCcy").is_none());
    }

    #[test]
    fn test_index_ticker_structure() {
        let ticker_json = json!({
            "instId": "BTC-USD",
            "idxPx": "43250.23",
            "high24h": "44000.00",
            "low24h": "42500.00",
            "open24h": "43000.00",
            "sodUtc0": "42800.00",
            "sodUtc8": "42900.00",
            "ts": "1597026383085"
        });

        let ticker: IndexTicker = serde_json::from_value(ticker_json).unwrap();
        assert_eq!(ticker.inst_id, "BTC-USD");
        assert_eq!(ticker.idx_px, "43250.23");
        assert_eq!(ticker.high_24h, "44000.00");
        assert_eq!(ticker.low_24h, "42500.00");
        assert_eq!(ticker.open_24h, "43000.00");
        assert_eq!(ticker.sod_utc0, "42800.00");
        assert_eq!(ticker.sod_utc8, "42900.00");
        assert_eq!(ticker.ts, "1597026383085");
    }

    #[test]
    fn test_get_index_tickers_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USD",
                    "idxPx": "43250.23",
                    "high24h": "44000.00",
                    "low24h": "42500.00",
                    "open24h": "43000.00",
                    "sodUtc0": "42800.00",
                    "sodUtc8": "42900.00",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetIndexTickersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD");
        assert_eq!(response.data.first().unwrap().idx_px, "43250.23");
    }

    #[test]
    fn test_index_ticker_serialization_roundtrip() {
        let original = GetIndexTickersRequest {
            quote_ccy: Some("USDT".to_string()),
            inst_id: None,
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetIndexTickersRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.quote_ccy, deserialized.quote_ccy);
        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_index_ticker_with_multiple_currencies() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USD",
                    "idxPx": "43250.23",
                    "high24h": "44000.00",
                    "low24h": "42500.00",
                    "open24h": "43000.00",
                    "sodUtc0": "42800.00",
                    "sodUtc8": "42900.00",
                    "ts": "1597026383085"
                },
                {
                    "instId": "ETH-USD",
                    "idxPx": "2650.45",
                    "high24h": "2700.00",
                    "low24h": "2600.00",
                    "open24h": "2625.00",
                    "sodUtc0": "2620.00",
                    "sodUtc8": "2630.00",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetIndexTickersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        
        let btc_ticker = &response.data[0];
        assert_eq!(btc_ticker.inst_id, "BTC-USD");
        assert_eq!(btc_ticker.idx_px, "43250.23");
        
        let eth_ticker = &response.data[1];
        assert_eq!(eth_ticker.inst_id, "ETH-USD");
        assert_eq!(eth_ticker.idx_px, "2650.45");
    }
}