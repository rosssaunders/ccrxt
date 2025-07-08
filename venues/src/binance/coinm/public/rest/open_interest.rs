use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Parameters for Open Interest
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenInterestRequest {
    /// Symbol name
    pub symbol: String,
}

/// Open interest data
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    /// Open interest
    pub open_interest: Decimal,
    /// Symbol name
    pub symbol: String,
    /// Timestamp
    pub time: i64,
}

impl RestClient {
    /// Get Open Interest
    ///
    /// Weight: 1
    pub async fn get_open_interest(&self, params: OpenInterestRequest) -> RestResult<OpenInterest> {
        self.send_request(
            "/dapi/v1/openInterest",
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}
