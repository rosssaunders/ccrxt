use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::Period, public::rest::RestClient};

/// Parameters for Long/Short Ratio
#[derive(Debug, Clone, Serialize)]
pub struct GlobalLongShortAccountRatioParams {
    /// Trading pair (e.g., "BTCUSD")
    pub pair: String,

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

/// Global long/short ratio
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLongShortAccountRatio {
    /// Pair name (not symbol)
    pub pair: String,

    /// Long account ratio
    pub long_account: Decimal,

    /// Short account ratio
    pub short_account: Decimal,

    /// Long/short ratio
    pub long_short_ratio: Decimal,

    /// Timestamp
    pub timestamp: i64,
}

impl RestClient {
    /// Get long/short ratio
    ///
    /// Weight: 1
    pub async fn get_global_long_short_account_ratio(
        &self,
        params: GlobalLongShortAccountRatioParams,
    ) -> RestResult<Vec<GlobalLongShortAccountRatio>> {
        self.send_request(
            "/futures/data/globalLongShortAccountRatio",
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}
