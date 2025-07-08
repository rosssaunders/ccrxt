//! Open Interest endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/openInterest
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for open interest.
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenInterestRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,
}

/// Represents an open interest response.
#[derive(Debug, Clone, Deserialize)]
pub struct OpenInterest {
    #[serde(rename = "openInterest")]
    pub open_interest: String,
    pub symbol: Cow<'static, str>,
    pub time: u64,
}

impl RestClient {
    /// Get present open interest of a specific symbol (GET /fapi/v1/openInterest)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest)
    pub async fn get_open_interest(&self, params: OpenInterestRequest) -> RestResult<OpenInterest> {
        let query = format!("symbol={}", params.symbol);
        self.send_request(
            "/fapi/v1/openInterest",
            reqwest::Method::GET,
            Some(&query),
            None,
            1,
        )
        .await
    }
}
