use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

/// Request parameters for exercise history
#[derive(Debug, Clone, Serialize, Default)]
pub struct ExerciseHistoryRequest {
    /// Underlying index like BTCUSDT
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records (Default: 100, Max: 100)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Exercise result type
#[derive(Debug, Clone, Deserialize)]
pub enum StrikeResult {
    /// Exercised
    #[serde(rename = "REALISTIC_VALUE_STRICKEN")]
    RealisticValueStricken,

    /// Expired OTM
    #[serde(rename = "EXTRINSIC_VALUE_EXPIRED")]
    ExtrinsicValueExpired,
}

/// Historical exercise record
#[derive(Debug, Clone, Deserialize)]
pub struct ExerciseHistoryRecord {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,

    /// Real strike price
    #[serde(rename = "realStrikePrice")]
    pub real_strike_price: Decimal,

    /// Exercise time
    #[serde(rename = "expiryDate")]
    pub expiry_date: u64,

    /// Strike result
    #[serde(rename = "strikeResult")]
    pub strike_result: StrikeResult,
}

impl RestClient {
    /// Get historical exercise records
    ///
    /// Returns historical exercise records.
    /// - REALISTIC_VALUE_STRICKEN -> Exercised
    /// - EXTRINSIC_VALUE_EXPIRED -> Expired OTM
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Historical-Exercise-Records)
    /// Method: GET /eapi/v1/exerciseHistory
    /// Weight: 3
    /// Security: None
    pub async fn get_exercise_history(
        &self,
        params: ExerciseHistoryRequest,
    ) -> RestResult<Vec<ExerciseHistoryRecord>> {
        let query_string = if params.underlying.is_some()
            || params.start_time.is_some()
            || params.end_time.is_some()
            || params.limit.is_some()
        {
            Some(serde_urlencoded::to_string(&params).map_err(|e| {
                crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
            })?)
        } else {
            None
        };

        self.send_request(
            "/eapi/v1/exerciseHistory",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            3,
        )
        .await
    }
}
