use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for asset index (Multi-Assets mode)
const ASSET_INDEX_ENDPOINT: &str = "/fapi/v1/assetIndex";

/// Request parameters for the asset index endpoint.
///
/// Used to query the asset index for Multi-Assets mode.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexRequest {
    /// Asset pair to query (e.g., "ADAUSD"). Optional.
    /// If not provided, returns all asset indices.
    ///
    /// Valid values: any supported asset pair symbol.
    ///
    /// Format: uppercase string, e.g., "BTCUSD".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a single asset index returned by the API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    /// Asset pair symbol (e.g., "ADAUSD").
    pub symbol: Cow<'static, str>,

    /// Timestamp in milliseconds since epoch.
    pub time: u64,

    /// Asset index value as a string.
    pub index: String,

    /// Bid buffer value.
    pub bid_buffer: String,

    /// Ask buffer value.
    pub ask_buffer: String,

    /// Bid rate value.
    pub bid_rate: String,

    /// Ask rate value.
    pub ask_rate: String,

    /// Auto-exchange bid buffer value.
    pub auto_exchange_bid_buffer: String,

    /// Auto-exchange ask buffer value.
    pub auto_exchange_ask_buffer: String,

    /// Auto-exchange bid rate value.
    pub auto_exchange_bid_rate: String,

    /// Auto-exchange ask rate value.
    pub auto_exchange_ask_rate: String,
}

/// Response type for asset index endpoint.
///
/// The API may return either a single asset index or a list of asset indices.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AssetIndexResult {
    /// Multiple asset indices returned as a list.
    Multiple(Vec<AssetIndex>),

    /// Single asset index returned.
    Single(AssetIndex),
}

impl RestClient {
    /// Get asset index for Multi-Assets mode (GET /fapi/v1/assetIndex)
    ///
    /// Retrieves the asset index for Multi-Assets mode.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `params` - Request parameters for asset index query
    ///
    /// # Returns
    /// AssetIndexResult - single or multiple asset indices
    pub async fn get_asset_index(&self, params: AssetIndexRequest) -> RestResult<AssetIndexResult> {
        self.send_get_request(ASSET_INDEX_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_index_request_serialization() {
        let request = AssetIndexRequest {
            symbol: Some("ADAUSD".into()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ADAUSD");
    }

    #[test]
    fn test_asset_index_request_no_symbol() {
        let request = AssetIndexRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_asset_index_response_deserialization() {
        let json = r#"{
            "symbol": "ADAUSD",
            "time": 1625184000000,
            "index": "0.50000000",
            "bidBuffer": "0.00100000",
            "askBuffer": "0.00100000",
            "bidRate": "0.00020000",
            "askRate": "0.00020000",
            "autoExchangeBidBuffer": "0.00050000",
            "autoExchangeAskBuffer": "0.00050000",
            "autoExchangeBidRate": "0.00010000",
            "autoExchangeAskRate": "0.00010000"
        }"#;

        let asset_index: AssetIndex = serde_json::from_str(json).unwrap();
        assert_eq!(asset_index.symbol, "ADAUSD");
        assert_eq!(asset_index.time, 1625184000000);
        assert_eq!(asset_index.index, "0.50000000");
        assert_eq!(asset_index.bid_buffer, "0.00100000");
        assert_eq!(asset_index.ask_buffer, "0.00100000");
        assert_eq!(asset_index.bid_rate, "0.00020000");
        assert_eq!(asset_index.ask_rate, "0.00020000");
        assert_eq!(asset_index.auto_exchange_bid_buffer, "0.00050000");
        assert_eq!(asset_index.auto_exchange_ask_buffer, "0.00050000");
        assert_eq!(asset_index.auto_exchange_bid_rate, "0.00010000");
        assert_eq!(asset_index.auto_exchange_ask_rate, "0.00010000");
    }

    #[test]
    fn test_asset_index_result_single_deserialization() {
        let json = r#"{
            "symbol": "ADAUSD",
            "time": 1625184000000,
            "index": "0.50000000",
            "bidBuffer": "0.00100000",
            "askBuffer": "0.00100000",
            "bidRate": "0.00020000",
            "askRate": "0.00020000",
            "autoExchangeBidBuffer": "0.00050000",
            "autoExchangeAskBuffer": "0.00050000",
            "autoExchangeBidRate": "0.00010000",
            "autoExchangeAskRate": "0.00010000"
        }"#;

        let result: AssetIndexResult = serde_json::from_str(json).unwrap();
        match result {
            AssetIndexResult::Single(asset_index) => {
                assert_eq!(asset_index.symbol, "ADAUSD");
                assert_eq!(asset_index.index, "0.50000000");
            }
            AssetIndexResult::Multiple(_) => panic!("Expected Single variant"),
        }
    }

    #[test]
    fn test_asset_index_result_multiple_deserialization() {
        let json = r#"[
            {
                "symbol": "ADAUSD",
                "time": 1625184000000,
                "index": "0.50000000",
                "bidBuffer": "0.00100000",
                "askBuffer": "0.00100000",
                "bidRate": "0.00020000",
                "askRate": "0.00020000",
                "autoExchangeBidBuffer": "0.00050000",
                "autoExchangeAskBuffer": "0.00050000",
                "autoExchangeBidRate": "0.00010000",
                "autoExchangeAskRate": "0.00010000"
            },
            {
                "symbol": "DOTUSD",
                "time": 1625184000000,
                "index": "25.00000000",
                "bidBuffer": "0.00200000",
                "askBuffer": "0.00200000",
                "bidRate": "0.00030000",
                "askRate": "0.00030000",
                "autoExchangeBidBuffer": "0.00100000",
                "autoExchangeAskBuffer": "0.00100000",
                "autoExchangeBidRate": "0.00020000",
                "autoExchangeAskRate": "0.00020000"
            }
        ]"#;

        let result: AssetIndexResult = serde_json::from_str(json).unwrap();
        match result {
            AssetIndexResult::Multiple(indices) => {
                assert_eq!(indices.len(), 2);
                assert_eq!(indices[0].symbol, "ADAUSD");
                assert_eq!(indices[0].index, "0.50000000");
                assert_eq!(indices[1].symbol, "DOTUSD");
                assert_eq!(indices[1].index, "25.00000000");
            }
            AssetIndexResult::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_asset_index_extreme_values() {
        let json = r#"{
            "symbol": "BTCUSD",
            "time": 1625184000000,
            "index": "99999.99999999",
            "bidBuffer": "0.99999999",
            "askBuffer": "0.99999999",
            "bidRate": "0.99999999",
            "askRate": "0.99999999",
            "autoExchangeBidBuffer": "0.00000001",
            "autoExchangeAskBuffer": "0.00000001",
            "autoExchangeBidRate": "0.00000001",
            "autoExchangeAskRate": "0.00000001"
        }"#;

        let asset_index: AssetIndex = serde_json::from_str(json).unwrap();
        assert_eq!(asset_index.index, "99999.99999999");
        assert_eq!(asset_index.bid_buffer, "0.99999999");
        assert_eq!(asset_index.auto_exchange_bid_rate, "0.00000001");
    }
}
