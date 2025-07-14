//! Asset Index endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/assetIndex
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for asset index.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AssetIndexRequest {
    /// Asset pair (e.g., "ADAUSD"). Optional.
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents an asset index response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
    pub symbol: Cow<'static, str>,
    pub time: u64,
    pub index: String,
    #[serde(rename = "bidBuffer")]
    pub bid_buffer: String,
    #[serde(rename = "askBuffer")]
    pub ask_buffer: String,
    #[serde(rename = "bidRate")]
    pub bid_rate: String,
    #[serde(rename = "askRate")]
    pub ask_rate: String,
    #[serde(rename = "autoExchangeBidBuffer")]
    pub auto_exchange_bid_buffer: String,
    #[serde(rename = "autoExchangeAskBuffer")]
    pub auto_exchange_ask_buffer: String,
    #[serde(rename = "autoExchangeBidRate")]
    pub auto_exchange_bid_rate: String,
    #[serde(rename = "autoExchangeAskRate")]
    pub auto_exchange_ask_rate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetIndexResult {
    Multiple(Vec<AssetIndex>),
    Single(AssetIndex),
}

impl RestClient {
    /// Get asset index for Multi-Assets mode (GET /fapi/v1/assetIndex)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index)
    pub async fn get_asset_index(&self, params: AssetIndexRequest) -> RestResult<AssetIndexResult> {
        let query = params.symbol.map(|s| format!("symbol={}", s));
        self.send_request(
            "/fapi/v1/assetIndex",
            reqwest::Method::GET,
            query.as_deref(),
            None,
            1,
        )
        .await
    }
}
