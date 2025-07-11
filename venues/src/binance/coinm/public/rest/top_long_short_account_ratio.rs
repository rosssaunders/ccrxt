use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::Period, public::rest::RestClient};

/// Parameters for Top Trader Long/Short Ratio (Accounts)
#[derive(Debug, Clone, Serialize)]
pub struct TopLongShortAccountRatioParams {
    /// Pair name (e.g., "BTCUSD")
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

/// Top trader long/short ratio (accounts)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortAccountRatio {
    /// Pair name
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
    /// Get top trader long/short ratio (accounts)
    ///
    /// Weight: 1
    pub async fn get_top_long_short_account_ratio(
        &self,
        params: TopLongShortAccountRatioParams,
    ) -> RestResult<Vec<TopLongShortAccountRatio>> {
        self.send_request(
            "/futures/data/topLongShortAccountRatio",
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}
