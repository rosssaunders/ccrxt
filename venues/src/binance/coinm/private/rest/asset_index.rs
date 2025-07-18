use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

const ASSET_INDEX_ENDPOINT: &str = "/dapi/v1/assetIndex";

/// Request parameters for asset index.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexRequest {
    /// Trading symbol (optional, all symbols returned if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Base asset information in the asset index.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseAssetList {
    /// Base asset
    pub base_asset: String,

    /// Weight in percentage
    pub weight_in_percentage: String,

    /// Weight in quantity
    pub weight_in_quantity: String,
}

/// Response for asset index.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexResponse {
    /// Symbol
    pub symbol: String,

    /// Time
    pub time: u64,

    /// Base asset list
    pub base_asset_list: Vec<BaseAssetList>,
}

impl RestClient {
    /// Get asset index on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/assetIndex
    /// Weight: 10
    /// Requires API key.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`AssetIndexRequest`])
    ///
    /// # Returns
    /// A list of [`AssetIndexResponse`] objects with asset index information.
    pub async fn get_asset_index(
        &self,
        params: AssetIndexRequest,
    ) -> RestResult<Vec<AssetIndexResponse>> {
        shared::send_signed_request(
            self,
            ASSET_INDEX_ENDPOINT,
            reqwest::Method::GET,
            params,
            10,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_index_request_serialization() {
        let request = AssetIndexRequest {
            symbol: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_asset_index_request_serialization_with_symbol() {
        let request = AssetIndexRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD_PERP");
    }

    #[test]
    fn test_asset_index_request_serialization_with_all_params() {
        let request = AssetIndexRequest {
            symbol: Some("ETHUSD_PERP".to_string()),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_base_asset_list_deserialization() {
        let json = r#"{
            "baseAsset": "BTC",
            "weightInPercentage": "25.00",
            "weightInQuantity": "0.50000000"
        }"#;

        let base_asset: BaseAssetList = serde_json::from_str(json).unwrap();
        assert_eq!(base_asset.base_asset, "BTC");
        assert_eq!(base_asset.weight_in_percentage, "25.00");
        assert_eq!(base_asset.weight_in_quantity, "0.50000000");
    }

    #[test]
    fn test_asset_index_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSD_PERP",
            "time": 1699948800000,
            "baseAssetList": [
                {
                    "baseAsset": "BTC",
                    "weightInPercentage": "50.00",
                    "weightInQuantity": "1.00000000"
                },
                {
                    "baseAsset": "ETH",
                    "weightInPercentage": "30.00",
                    "weightInQuantity": "10.00000000"
                },
                {
                    "baseAsset": "BNB",
                    "weightInPercentage": "20.00",
                    "weightInQuantity": "100.00000000"
                }
            ]
        }"#;

        let response: AssetIndexResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.time, 1699948800000);
        assert_eq!(response.base_asset_list.len(), 3);
        
        assert_eq!(response.base_asset_list[0].base_asset, "BTC");
        assert_eq!(response.base_asset_list[0].weight_in_percentage, "50.00");
        assert_eq!(response.base_asset_list[0].weight_in_quantity, "1.00000000");
        
        assert_eq!(response.base_asset_list[1].base_asset, "ETH");
        assert_eq!(response.base_asset_list[1].weight_in_percentage, "30.00");
        assert_eq!(response.base_asset_list[1].weight_in_quantity, "10.00000000");
        
        assert_eq!(response.base_asset_list[2].base_asset, "BNB");
        assert_eq!(response.base_asset_list[2].weight_in_percentage, "20.00");
        assert_eq!(response.base_asset_list[2].weight_in_quantity, "100.00000000");
    }

    #[test]
    fn test_asset_index_response_list_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "time": 1699948800000,
                "baseAssetList": [
                    {
                        "baseAsset": "BTC",
                        "weightInPercentage": "100.00",
                        "weightInQuantity": "1.00000000"
                    }
                ]
            },
            {
                "symbol": "ETHUSD_PERP",
                "time": 1699948800000,
                "baseAssetList": [
                    {
                        "baseAsset": "ETH",
                        "weightInPercentage": "100.00",
                        "weightInQuantity": "15.00000000"
                    }
                ]
            }
        ]"#;

        let responses: Vec<AssetIndexResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 2);
        assert_eq!(responses[0].symbol, "BTCUSD_PERP");
        assert_eq!(responses[1].symbol, "ETHUSD_PERP");
    }
}
