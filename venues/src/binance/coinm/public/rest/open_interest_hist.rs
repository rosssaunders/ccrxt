use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::Period;
use crate::binance::coinm::public::rest::RestClient;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Parameters for Open Interest Statistics
#[derive(Debug, Clone, Serialize)]
pub struct OpenInterestHistParams {
    /// Symbol name
    pub symbol: String,
    /// The time interval
    pub period: Period,
    /// Maximum 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Start time
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// Open interest statistics
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHist {
    /// Symbol name
    pub symbol: String,
    /// Sum of open interest
    pub sum_open_interest: Decimal,
    /// Sum of open interest value
    pub sum_open_interest_value: Decimal,
    /// Timestamp
    pub timestamp: i64,
}

impl RestClient {
    /// Get open interest statistics
    ///
    /// Weight: 1
    pub async fn get_open_interest_hist(&self, params: OpenInterestHistParams) -> RestResult<Vec<OpenInterestHist>> {
        self.send_request(
            "/futures/data/openInterestHist",
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}
