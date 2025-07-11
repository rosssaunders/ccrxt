use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::Period, public::rest::RestClient};

/// Parameters for Top Trader Long/Short Ratio (Positions)
#[derive(Debug, Clone, Serialize)]
pub struct TopLongShortPositionRatioParams {
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

/// Top trader long/short ratio (positions)
#[derive(Debug, Clone, Deserialize)]
pub struct TopLongShortPositionRatio {
    /// Pair name (e.g., "BTCUSD")
    pub pair: String,
    /// Long position ratio
    #[serde(rename = "longPosition")]
    pub long_position: Decimal,
    /// Short position ratio
    #[serde(rename = "shortPosition")]
    pub short_position: Decimal,
    /// Long short ratio
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Decimal,
    /// Timestamp
    pub timestamp: i64,
}

impl RestClient {
    /// Get top trader long/short ratio (positions)
    ///
    /// Weight: 1
    pub async fn get_top_long_short_position_ratio(
        &self,
        params: TopLongShortPositionRatioParams,
    ) -> RestResult<Vec<TopLongShortPositionRatio>> {
        self.send_request(
            "/futures/data/topLongShortPositionRatio",
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}
