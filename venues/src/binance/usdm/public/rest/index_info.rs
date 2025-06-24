//! Index Info endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/indexInfo
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information)

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for index info.
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct IndexInfoRequest {
    /// Composite index symbol. Optional.
    #[serde(rename = "symbol")]
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a base asset in the index info response.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct IndexBaseAsset {
    /// The base asset symbol.
    #[serde(rename = "baseAsset")]
    pub base_asset: Cow<'static, str>,

    /// The quote asset symbol.
    #[serde(rename = "quoteAsset")]
    pub quote_asset: Cow<'static, str>,

    /// The weight in quantity as a string.
    #[serde(rename = "weightInQuantity")]
    pub weight_in_quantity: Cow<'static, str>,

    /// The weight in percentage as a string.
    #[serde(rename = "weightInPercentage")]
    pub weight_in_percentage: Cow<'static, str>,
}

/// Represents an index info response.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct IndexInfo {
    /// The composite index symbol.
    #[serde(rename = "symbol")]
    pub symbol: Cow<'static, str>,

    /// The timestamp in milliseconds.
    #[serde(rename = "time")]
    pub time: u64,

    /// The component name.
    #[serde(rename = "component")]
    pub component: Cow<'static, str>,

    /// The list of base assets in the index.
    #[serde(rename = "baseAssetList")]
    pub base_asset_list: Vec<IndexBaseAsset>,
}

impl RestClient {
    /// Query composite index symbol information (GET /fapi/v1/indexInfo)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information)
    pub async fn get_index_info(&self, params: IndexInfoRequest) -> RestResult<Vec<IndexInfo>> {
        let query = params.symbol.map(|s| format!("symbol={}", s));
        self.send_request(
            "/fapi/v1/indexInfo",
            reqwest::Method::GET,
            query.as_deref(),
            None,
            1,
        )
        .await
    }
}
