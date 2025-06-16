use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for getting index components
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexComponentsRequest {
    /// Index name (e.g., "BTC-USDT")
    pub index: String,
}

/// Individual component details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponent {
    /// Name of Exchange
    pub exch: String,
    /// Name of Exchange Trading Pairs
    pub symbol: String,
    /// Price of Exchange Trading Pairs
    #[serde(rename = "symPx")]
    pub sym_px: String,
    /// Weights
    pub wgt: String,
    /// Price converted to index
    #[serde(rename = "cnvPx")]
    pub cnv_px: String,
}

/// Individual index component information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponentData {
    /// Index
    pub index: String,
    /// Latest Index Price
    pub last: String,
    /// Data generation time, Unix timestamp format in milliseconds
    pub ts: String,
    /// Components
    pub components: Vec<IndexComponent>,
}

/// Response for getting index components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIndexComponentsResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Index component data
    pub data: Vec<IndexComponentData>,
}

impl RestClient {
    /// Get index component information data on the market
    ///
    /// Get the index component information data on the market.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-index-components
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The index components request parameters
    ///
    /// # Returns
    /// Response containing the index component information
    pub async fn get_index_components(&self, request: &GetIndexComponentsRequest) -> RestResult<GetIndexComponentsResponse> {
        self.send_request(
            "api/v5/market/index-components",
            reqwest::Method::GET,
            Some(request),
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
    fn test_get_index_components_request_structure() {
        let request = GetIndexComponentsRequest {
            index: "BTC-USDT".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("index").and_then(|v| v.as_str()),
            Some("BTC-USDT")
        );
    }

    #[test]
    fn test_index_component_structure() {
        let component_json = json!({
            "exch": "Binance",
            "symbol": "BTCUSDT",
            "symPx": "50000.0",
            "wgt": "0.25",
            "cnvPx": "50000.0"
        });

        let component: IndexComponent = serde_json::from_value(component_json).unwrap();
        assert_eq!(component.exch, "Binance");
        assert_eq!(component.symbol, "BTCUSDT");
        assert_eq!(component.sym_px, "50000.0");
        assert_eq!(component.wgt, "0.25");
        assert_eq!(component.cnv_px, "50000.0");
    }

    #[test]
    fn test_index_component_data_structure() {
        let data_json = json!({
            "index": "BTC-USDT",
            "last": "50000.0",
            "ts": "1597026383085",
            "components": [
                {
                    "exch": "Binance",
                    "symbol": "BTCUSDT",
                    "symPx": "50000.0",
                    "wgt": "0.25",
                    "cnvPx": "50000.0"
                }
            ]
        });

        let data: IndexComponentData = serde_json::from_value(data_json).unwrap();
        assert_eq!(data.index, "BTC-USDT");
        assert_eq!(data.last, "50000.0");
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.components.len(), 1);
        assert_eq!(data.components[0].exch, "Binance");
        assert_eq!(data.components[0].symbol, "BTCUSDT");
    }

    #[test]
    fn test_get_index_components_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "index": "BTC-USDT",
                    "last": "50000.0",
                    "ts": "1597026383085",
                    "components": [
                        {
                            "exch": "Binance",
                            "symbol": "BTCUSDT",
                            "symPx": "50000.0",
                            "wgt": "0.25",
                            "cnvPx": "50000.0"
                        },
                        {
                            "exch": "Coinbase",
                            "symbol": "BTC-USD",
                            "symPx": "49950.0",
                            "wgt": "0.30",
                            "cnvPx": "49950.0"
                        }
                    ]
                }
            ]
        });

        let response: GetIndexComponentsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].index, "BTC-USDT");
        assert_eq!(response.data[0].last, "50000.0");
        assert_eq!(response.data[0].ts, "1597026383085");
        assert_eq!(response.data[0].components.len(), 2);
        assert_eq!(response.data[0].components[0].exch, "Binance");
        assert_eq!(response.data[0].components[1].exch, "Coinbase");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = GetIndexComponentsRequest {
            index: "BTC-USDT".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetIndexComponentsRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.index, deserialized.index);
    }

    #[test]
    fn test_index_component_data_with_empty_components() {
        let data_json = json!({
            "index": "BTC-USDT",
            "last": "50000.0",
            "ts": "1597026383085",
            "components": []
        });

        let data: IndexComponentData = serde_json::from_value(data_json).unwrap();
        assert_eq!(data.index, "BTC-USDT");
        assert_eq!(data.last, "50000.0");
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.components.len(), 0);
    }
}
