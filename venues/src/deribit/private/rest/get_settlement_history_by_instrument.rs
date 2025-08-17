use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const GET_SETTLEMENT_HISTORY_BY_INSTRUMENT_ENDPOINT: &str =
    "private/get_settlement_history_by_instrument";

/// REST API endpoint constant
/// Request for /private/get_settlement_history_by_instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettlementHistoryByInstrumentRequest {
    /// The instrument name to filter by
    pub instrument_name: String,

    /// Settlement type to filter by (e.g., funding, delivery) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Number of items to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Continuation token for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,

    /// Filter results starting from this timestamp (milliseconds since epoch) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_start_timestamp: Option<u64>,
}

/// Settlement event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementEvent {
    /// Amount funded (optional)
    pub funded: Option<f64>,

    /// Current funding rate (optional)
    pub funding: Option<f64>,

    /// Index price at event time (optional)
    pub index_price: Option<f64>,

    /// Instrument name (optional)
    pub instrument_name: Option<String>,

    /// Mark price at event time (optional)
    pub mark_price: Option<f64>,

    /// Position size (optional)
    pub position: Option<f64>,

    /// Profit or loss (optional)
    pub profit_loss: Option<f64>,

    /// Session bankruptcy amount (optional)
    pub session_bankruptcy: Option<f64>,

    /// Session profit or loss (optional)
    pub session_profit_loss: Option<f64>,

    /// Session tax (optional)
    pub session_tax: Option<f64>,

    /// Session tax rate (optional)
    pub session_tax_rate: Option<f64>,

    /// Socialized loss/gain (optional)
    pub socialized: Option<f64>,

    /// Event timestamp (milliseconds since epoch)
    pub timestamp: u64,

    /// Event type
    pub r#type: String,
}

/// Response for /private/get_settlement_history_by_instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettlementHistoryByInstrumentResult {
    /// Continuation token for pagination, if more results are available
    pub continuation: Option<String>,

    /// List of settlement events
    pub settlements: Vec<SettlementEvent>,
}

pub type GetSettlementHistoryByInstrumentResponse =
    JsonRpcResult<GetSettlementHistoryByInstrumentResult>;

impl RestClient {
    /// Retrieves public settlement, delivery and bankruptcy events filtered by instrument name.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_settlement_history_by_instrument)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - Parameters including instrument name and optional filters
    ///
    /// # Returns
    /// Settlement history events and optional continuation token
    pub async fn get_settlement_history_by_instrument(
        &self,
        request: GetSettlementHistoryByInstrumentRequest,
    ) -> RestResult<GetSettlementHistoryByInstrumentResponse> {
        self.send_signed_request(
            GET_SETTLEMENT_HISTORY_BY_INSTRUMENT_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}
