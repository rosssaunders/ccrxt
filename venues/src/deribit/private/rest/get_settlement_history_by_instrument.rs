//! Implementation for Deribit /private/get_settlement_history_by_instrument endpoint
//!
//! Retrieves public settlement, delivery and bankruptcy events filtered by instrument name.
//!
//! See Deribit API docs: https://docs.deribit.com/#private-get_settlement_history_by_instrument

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// Request for /private/get_settlement_history_by_instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettlementHistoryByInstrumentRequest {
    pub instrument_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_start_timestamp: Option<u64>,
}

/// Settlement event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementEvent {
    pub funded: Option<f64>,
    pub funding: Option<f64>,
    pub index_price: Option<f64>,
    pub instrument_name: Option<String>,
    pub mark_price: Option<f64>,
    pub position: Option<f64>,
    pub profit_loss: Option<f64>,
    pub session_bankruptcy: Option<f64>,
    pub session_profit_loss: Option<f64>,
    pub session_tax: Option<f64>,
    pub session_tax_rate: Option<f64>,
    pub socialized: Option<f64>,
    pub timestamp: u64,
    pub r#type: String,
}

/// Response for /private/get_settlement_history_by_instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettlementHistoryByInstrumentResponse {
    pub continuation: Option<String>,
    pub settlements: Vec<SettlementEvent>,
}

impl RestClient {
    /// Retrieves public settlement, delivery and bankruptcy events filtered by instrument name.
    pub async fn get_settlement_history_by_instrument(
        &self,
        request: GetSettlementHistoryByInstrumentRequest,
    ) -> RestResult<GetSettlementHistoryByInstrumentResponse> {
        self.send_signed_request(
            "private/get_settlement_history_by_instrument",
            &request,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}
