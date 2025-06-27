use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::{ContractType, Period};
use crate::binance::coinm::public::rest::RestClient;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Parameters for Basis
#[derive(Debug, Clone, Serialize)]
pub struct BasisRequest {
    /// Pair name
    pub pair: String,

    /// Contract type
    #[serde(rename = "contractType")]
    pub contract_type: ContractType,

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

/// Basis data
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Basis {
    /// Pair name
    pub pair: String,
    /// Contract type
    pub contract_type: ContractType,
    /// Futures price
    pub futures_price: Decimal,
    /// Index price
    pub index_price: Decimal,
    /// Basis
    pub basis: Decimal,
    /// Basis rate
    pub basis_rate: Decimal,
    /// Timestamp
    pub timestamp: i64,
}

impl RestClient {
    /// Get basis
    ///
    /// Weight: 1
    pub async fn get_basis(&self, params: BasisRequest) -> RestResult<Vec<Basis>> {
        self.send_request("/futures/data/basis", reqwest::Method::GET, Some(params), 1)
            .await
    }
}
