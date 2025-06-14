use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Request parameters for getting index tickers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexTickersRequest {
    /// Quote currency (optional)
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: Option<String>,
    /// Index (e.g., "BTC-USD")
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
}

/// Index ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexTicker {
    /// Index
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Latest index price
    #[serde(rename = "idxPx")]
    pub idx_px: String,
    /// Highest price in the past 24 hours
    #[serde(rename = "high24h")]
    pub high24h: String,
    /// Lowest price in the past 24 hours
    #[serde(rename = "low24h")]
    pub low24h: String,
    /// Open price in the past 24 hours
    #[serde(rename = "open24h")]
    pub open24h: String,
    /// Open price in the UTC 0
    #[serde(rename = "sodUtc0")]
    pub sod_utc0: String,
    /// Open price in the UTC 8
    #[serde(rename = "sodUtc8")]
    pub sod_utc8: String,
    /// Index price update time (Unix timestamp in milliseconds)
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
    /// Response containing the index ticker information
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
            inst_id: Some("BTC-USD".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("quoteCcy").and_then(|v| v.as_str()), Some("USD"));
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USD"));
    }

    #[test]
    fn test_index_ticker_structure() {
        let index_ticker_json = json!({
            "instId": "BTC-USD",
            "idxPx": "31000.5",
            "high24h": "32000.0",
            "low24h": "30000.0",
            "open24h": "30500.0",
            "sodUtc0": "30200.0",
            "sodUtc8": "30100.0",
            "ts": "1597026383085"
        });

        let index_ticker: IndexTicker = serde_json::from_value(index_ticker_json).unwrap();
        assert_eq!(index_ticker.inst_id, "BTC-USD");
        assert_eq!(index_ticker.idx_px, "31000.5");
        assert_eq!(index_ticker.high24h, "32000.0");
        assert_eq!(index_ticker.low24h, "30000.0");
    }

    #[test]
    fn test_get_index_tickers_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USD",
                    "idxPx": "31000.5",
                    "high24h": "32000.0",
                    "low24h": "30000.0",
                    "open24h": "30500.0",
                    "sodUtc0": "30200.0",
                    "sodUtc8": "30100.0",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetIndexTickersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        let index_ticker = response.data.first().unwrap();
        assert_eq!(index_ticker.inst_id, "BTC-USD");
        assert_eq!(index_ticker.idx_px, "31000.5");
    }
}