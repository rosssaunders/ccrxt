//! Basis (GET /futures/data/basis)
//!
//! See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis

use crate::binance::usdm::Errors;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::{ContractType, Period};

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for the Basis endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct BasisRequest<'a> {
    /// The pair to query (e.g., "BTCUSDT").
    pub pair: Cow<'a, str>,

    /// The contract type (PERPETUAL, CURRENT_QUARTER, NEXT_QUARTER).
    pub contract_type: ContractType,

    /// The period interval (e.g., "5m", "1h").
    pub period: Period,

    /// Number of data points to return (default 30, max 500).
    pub limit: u32,

    /// Start time in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasisResponse<'a> {
    pub index_price: Cow<'a, str>,
    pub contract_type: ContractType,
    pub basis_rate: Cow<'a, str>,
    pub futures_price: Cow<'a, str>,
    pub annualized_basis_rate: Cow<'a, str>,
    pub basis: Cow<'a, str>,
    pub pair: Cow<'a, str>,
    pub timestamp: u64,
}

impl crate::binance::usdm::public::rest::RestClient {
    /// Query future basis (GET /futures/data/basis)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis)
    pub async fn basis<'a>(&self, params: BasisRequest<'a>) -> RestResult<Vec<BasisResponse<'a>>> {
        let endpoint = "/futures/data/basis";
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize params: {e}")))?;
        let resp = self
            .send_request::<Vec<BasisResponse>>(
                endpoint,
                reqwest::Method::GET,
                Some(&query),
                None,
                0,
            )
            .await?;
        Ok(resp)
    }
}
