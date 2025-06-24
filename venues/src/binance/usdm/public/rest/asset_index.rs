//! Asset Index endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/assetIndex
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index)

use super::RestClient;
use crate::binance::usdm::RestResult;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for asset index.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AssetIndexRequest {
    /// Asset pair (e.g., "ADAUSD"). Optional.
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents an asset index response.
#[derive(Debug, Clone, Deserialize)]
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

impl RestClient {
    /// Get asset index for Multi-Assets mode (GET /fapi/v1/assetIndex)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index)
    pub async fn get_asset_index(&self, params: AssetIndexRequest) -> RestResult<Vec<AssetIndex>> {
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
