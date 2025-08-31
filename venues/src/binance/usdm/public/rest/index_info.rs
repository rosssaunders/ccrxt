use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::{RestResult, public_client::RestClient};

const INDEX_INFO_ENDPOINT: &str = "/fapi/v1/indexInfo";

/// Request parameters for querying composite index symbol information.
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct IndexInfoRequest {
    /// Composite index symbol. Optional - if not provided, returns information for all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a base asset component in a composite index.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IndexBaseAsset {
    /// The base asset symbol (e.g., "SUSHI", "UNI").
    pub base_asset: Cow<'static, str>,

    /// The quote asset symbol (e.g., "USDT").
    pub quote_asset: Cow<'static, str>,

    /// The weight of this asset in quantity as a decimal string.
    pub weight_in_quantity: Cow<'static, str>,

    /// The weight of this asset as a percentage of the total index as a decimal string.
    pub weight_in_percentage: Cow<'static, str>,
}

/// Represents composite index symbol information.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IndexInfo {
    /// The composite index symbol (e.g., "DEFIUSDT").
    pub symbol: Cow<'static, str>,

    /// The timestamp in milliseconds since Unix epoch when this information was generated.
    pub time: u64,

    /// The component name describing this index (e.g., "baseAsset").
    pub component: Cow<'static, str>,

    /// List of base assets that make up this composite index with their weights.
    pub base_asset_list: Vec<IndexBaseAsset>,
}

/// Response wrapper for index info that can be either a single object or an array.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum IndexInfoResponse {
    /// Single index information object when querying a specific symbol.
    Single(IndexInfo),

    /// Multiple index information objects when querying all symbols or multiple results.
    Multiple(Vec<IndexInfo>),
}

impl RestClient {
    /// Composite Index Symbol Information
    ///
    /// Query composite index symbol information.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The index info request parameters
    ///
    /// # Returns
    /// Index information - either single or multiple objects depending on query
    pub async fn get_index_info(&self, request: IndexInfoRequest) -> RestResult<IndexInfoResponse> {
        self.send_get_request(INDEX_INFO_ENDPOINT, Some(request), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_info_request_serialization_with_symbol() {
        let request = IndexInfoRequest {
            symbol: Some("DEFIUSDT".into()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=DEFIUSDT");
    }

    #[test]
    fn test_index_info_request_serialization_without_symbol() {
        let request = IndexInfoRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_index_base_asset_deserialization() {
        let json = r#"{
            "baseAsset": "SUSHI",
            "quoteAsset": "USDT",
            "weightInQuantity": "10000",
            "weightInPercentage": "10.00"
        }"#;

        let base_asset: IndexBaseAsset = serde_json::from_str(json).unwrap();
        assert_eq!(base_asset.base_asset, "SUSHI");
        assert_eq!(base_asset.quote_asset, "USDT");
        assert_eq!(base_asset.weight_in_quantity, "10000");
        assert_eq!(base_asset.weight_in_percentage, "10.00");
    }

    #[test]
    fn test_index_info_deserialization() {
        let json = r#"{
            "symbol": "DEFIUSDT",
            "time": 1625184000000,
            "component": "DeFi Index",
            "baseAssetList": [
                {
                    "baseAsset": "SUSHI",
                    "quoteAsset": "USDT",
                    "weightInQuantity": "10000",
                    "weightInPercentage": "10.00"
                },
                {
                    "baseAsset": "UNI",
                    "quoteAsset": "USDT",
                    "weightInQuantity": "2000",
                    "weightInPercentage": "20.00"
                },
                {
                    "baseAsset": "AAVE",
                    "quoteAsset": "USDT",
                    "weightInQuantity": "100",
                    "weightInPercentage": "30.00"
                }
            ]
        }"#;

        let index_info: IndexInfo = serde_json::from_str(json).unwrap();
        assert_eq!(index_info.symbol, "DEFIUSDT");
        assert_eq!(index_info.time, 1625184000000);
        assert_eq!(index_info.component, "DeFi Index");
        assert_eq!(index_info.base_asset_list.len(), 3);

        assert_eq!(index_info.base_asset_list[0].base_asset, "SUSHI");
        assert_eq!(index_info.base_asset_list[0].weight_in_percentage, "10.00");

        assert_eq!(index_info.base_asset_list[1].base_asset, "UNI");
        assert_eq!(index_info.base_asset_list[1].weight_in_percentage, "20.00");

        assert_eq!(index_info.base_asset_list[2].base_asset, "AAVE");
        assert_eq!(index_info.base_asset_list[2].weight_in_percentage, "30.00");
    }

    #[test]
    fn test_index_info_result_single_deserialization() {
        let json = r#"{
            "symbol": "DEFIUSDT",
            "time": 1625184000000,
            "component": "DeFi Index",
            "baseAssetList": [
                {
                    "baseAsset": "SUSHI",
                    "quoteAsset": "USDT",
                    "weightInQuantity": "10000",
                    "weightInPercentage": "100.00"
                }
            ]
        }"#;

        let result: IndexInfoResponse = serde_json::from_str(json).unwrap();
        if let IndexInfoResponse::Single(info) = result {
            assert_eq!(info.symbol, "DEFIUSDT");
            assert_eq!(info.base_asset_list.len(), 1);
        } else {
            unreachable!("Expected Single variant");
        }
    }

    #[test]
    fn test_index_info_result_multiple_deserialization() {
        let json = r#"[
            {
                "symbol": "DEFIUSDT",
                "time": 1625184000000,
                "component": "DeFi Index",
                "baseAssetList": [
                    {
                        "baseAsset": "SUSHI",
                        "quoteAsset": "USDT",
                        "weightInQuantity": "10000",
                        "weightInPercentage": "50.00"
                    },
                    {
                        "baseAsset": "UNI",
                        "quoteAsset": "USDT",
                        "weightInQuantity": "2000",
                        "weightInPercentage": "50.00"
                    }
                ]
            },
            {
                "symbol": "NFTUSDT",
                "time": 1625184000000,
                "component": "NFT Index",
                "baseAssetList": [
                    {
                        "baseAsset": "AXS",
                        "quoteAsset": "USDT",
                        "weightInQuantity": "1000",
                        "weightInPercentage": "100.00"
                    }
                ]
            }
        ]"#;

        let result: IndexInfoResponse = serde_json::from_str(json).unwrap();
        if let IndexInfoResponse::Multiple(indices) = result {
            assert_eq!(indices.len(), 2);
            assert_eq!(indices[0].symbol, "DEFIUSDT");
            assert_eq!(indices[0].component, "DeFi Index");
            assert_eq!(indices[1].symbol, "NFTUSDT");
            assert_eq!(indices[1].component, "NFT Index");
        } else {
            unreachable!("Expected Multiple variant");
        }
    }

    #[test]
    fn test_index_info_empty_base_asset_list() {
        let json = r#"{
            "symbol": "EMPTYUSDT",
            "time": 1625184000000,
            "component": "Empty Index",
            "baseAssetList": []
        }"#;

        let index_info: IndexInfo = serde_json::from_str(json).unwrap();
        assert_eq!(index_info.symbol, "EMPTYUSDT");
        assert_eq!(index_info.base_asset_list.len(), 0);
    }

    #[test]
    fn test_index_base_asset_high_precision() {
        let json = r#"{
            "baseAsset": "DOGE",
            "quoteAsset": "USDT",
            "weightInQuantity": "123456.789012",
            "weightInPercentage": "12.345678"
        }"#;

        let base_asset: IndexBaseAsset = serde_json::from_str(json).unwrap();
        assert_eq!(base_asset.weight_in_quantity, "123456.789012");
        assert_eq!(base_asset.weight_in_percentage, "12.345678");
    }

    #[test]
    fn test_index_info_request_default() {
        let request = IndexInfoRequest::default();
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_index_info_weights_sum() {
        let json = r#"{
            "symbol": "DEFIUSDT",
            "time": 1625184000000,
            "component": "DeFi Index",
            "baseAssetList": [
                {
                    "baseAsset": "SUSHI",
                    "quoteAsset": "USDT",
                    "weightInQuantity": "10000",
                    "weightInPercentage": "25.00"
                },
                {
                    "baseAsset": "UNI",
                    "quoteAsset": "USDT",
                    "weightInQuantity": "2000",
                    "weightInPercentage": "25.00"
                },
                {
                    "baseAsset": "AAVE",
                    "quoteAsset": "USDT",
                    "weightInQuantity": "100",
                    "weightInPercentage": "25.00"
                },
                {
                    "baseAsset": "COMP",
                    "quoteAsset": "USDT",
                    "weightInQuantity": "50",
                    "weightInPercentage": "25.00"
                }
            ]
        }"#;

        let index_info: IndexInfo = serde_json::from_str(json).unwrap();
        let total_weight: f64 = index_info
            .base_asset_list
            .iter()
            .map(|asset| asset.weight_in_percentage.parse::<f64>().unwrap())
            .sum();
        assert_eq!(total_weight, 100.0);
    }

    #[test]
    fn test_index_info_request_serialization_empty() {
        let request = IndexInfoRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_index_base_asset_zero_weights() {
        let json = r#"{
            "baseAsset": "TEST",
            "quoteAsset": "USDT",
            "weightInQuantity": "0.0",
            "weightInPercentage": "0.0"
        }"#;

        let base_asset: IndexBaseAsset = serde_json::from_str(json).unwrap();
        assert_eq!(base_asset.weight_in_quantity, "0.0");
        assert_eq!(base_asset.weight_in_percentage, "0.0");
    }

    #[test]
    fn test_index_info_response_serialization_roundtrip() {
        let original = IndexInfo {
            symbol: "TESTUSDT".into(),
            time: 1625184000000,
            component: "Test Index".into(),
            base_asset_list: vec![IndexBaseAsset {
                base_asset: "BTC".into(),
                quote_asset: "USDT".into(),
                weight_in_quantity: "1.0".into(),
                weight_in_percentage: "100.0".into(),
            }],
        };

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: IndexInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }
}
