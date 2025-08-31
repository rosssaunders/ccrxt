use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::{OptionsContractType, RestResult, private_client::RestClient};

const GET_EXERCISE_RECORD_ENDPOINT: &str = "/eapi/v1/exerciseRecord";

/// Request parameters for querying exercise records
#[derive(Debug, Clone, Serialize, Default)]
pub struct ExerciseRecordRequest {
    /// Option trading pair (if omitted, returns all exercise records)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of result sets returned (default: 100, max: 1000)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Exercise record
#[derive(Debug, Clone, Deserialize)]
pub struct ExerciseRecord {
    /// Exercise ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Asset type
    #[serde(rename = "currency")]
    pub currency: String,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Position change
    #[serde(rename = "exercisePrice")]
    pub exercise_price: Decimal,

    /// Mark price at exercise
    #[serde(rename = "markPrice")]
    pub mark_price: Decimal,

    /// Exercise quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Exercise amount
    #[serde(rename = "amount")]
    pub amount: Decimal,

    /// Exercise fee
    #[serde(rename = "fee")]
    pub fee: Decimal,

    /// Exercise time
    #[serde(rename = "createDate")]
    pub create_date: u64,

    /// Price scale
    #[serde(rename = "priceScale")]
    pub price_scale: u32,

    /// Quantity scale
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,

    /// Option side (CALL or PUT)
    #[serde(rename = "optionSide")]
    pub option_side: OptionsContractType,

    /// Position direction: 'LONG' or 'SHORT'
    #[serde(rename = "direction")]
    pub direction: String,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,
}

impl RestClient {
    /// Query exercise records
    ///
    /// Returns exercise records for options.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/trade/Option-Exercise-Record)
    ///
    /// Method: GET /eapi/v1/exerciseRecord
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_exercise_record(
        &self,
        params: ExerciseRecordRequest,
    ) -> RestResult<Vec<ExerciseRecord>> {
        self.send_get_signed_request(GET_EXERCISE_RECORD_ENDPOINT, params, 1, false)
            .await
    }
}
