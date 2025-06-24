//! Constituents endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/constituents
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents)

use super::RestClient;
use crate::binance::usdm::RestResult;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for index constituents.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ConstituentsRequest {
    /// Symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,
}

/// Represents a single constituent in the index.
#[derive(Debug, Clone, Deserialize)]
pub struct Constituent {
    pub exchange: Cow<'static, str>,
    pub symbol: Cow<'static, str>,
    pub price: String,
    pub weight: String,
}

/// Represents the index constituents response.
#[derive(Debug, Clone, Deserialize)]
pub struct ConstituentsResponse {
    pub symbol: Cow<'static, str>,
    pub time: u64,
    pub constituents: Vec<Constituent>,
}

impl RestClient {
    /// Query index price constituents (GET /fapi/v1/constituents)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents)
    pub async fn get_constituents(&self, params: ConstituentsRequest) -> RestResult<ConstituentsResponse> {
        let query = format!("symbol={}", params.symbol);
        self.send_request(
            "/fapi/v1/constituents",
            reqwest::Method::GET,
            Some(&query),
            None,
            2,
        )
        .await
    }
}
