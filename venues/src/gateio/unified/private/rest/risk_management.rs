use serde::{Deserialize, Serialize};

use super::RestClient;

const RISK_UNITS_ENDPOINT: &str = "/unified/risk_units";
const ESTIMATE_RATE_ENDPOINT: &str = "/unified/estimate_rate";
const HISTORY_LOAN_RATE_ENDPOINT: &str = "/unified/history_loan_rate";

/// Risk unit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskUnit {
    /// Currency
    pub currency: String,

    /// Spot hedge required
    pub spot_hedge_required: bool,

    /// Futures hedge required
    pub futures_hedge_required: bool,

    /// Options hedge required
    pub options_hedge_required: bool,
}

/// Request parameters for estimate rate
#[derive(Debug, Clone, Serialize)]
pub struct EstimateRateRequest {
    /// Currencies
    pub currencies: Vec<String>,
}

/// Rate estimate response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateEstimate {
    /// Currency
    pub currency: String,

    /// Estimated rate
    pub rate: String,
}

/// Request parameters for historical loan rates
#[derive(Debug, Clone, Serialize, Default)]
pub struct HistoricalLoanRateRequest {
    /// Currency
    pub currency: String,

    /// Start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Historical loan rate record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalLoanRate {
    /// Timestamp
    pub time: i64,

    /// Currency
    pub currency: String,

    /// Loan rate
    pub rate: String,
}

impl RestClient {
    /// Get risk units
    ///
    /// This endpoint returns risk unit configuration.
    pub async fn get_risk_units(&self) -> crate::gateio::unified::RestResult<Vec<RiskUnit>> {
        self.get(RISK_UNITS_ENDPOINT).await
    }

    /// Get estimated rates
    ///
    /// This endpoint returns estimated borrowing rates for currencies.
    pub async fn get_estimate_rate(
        &self,
        request: EstimateRateRequest,
    ) -> crate::gateio::unified::RestResult<Vec<RateEstimate>> {
        self.post(ESTIMATE_RATE_ENDPOINT, &request).await
    }

    /// Get historical loan rates
    ///
    /// This endpoint returns historical borrowing rates.
    pub async fn get_history_loan_rate(
        &self,
        params: HistoricalLoanRateRequest,
    ) -> crate::gateio::unified::RestResult<Vec<HistoricalLoanRate>> {
        self.get_with_query(HISTORY_LOAN_RATE_ENDPOINT, &params)
            .await
    }
}
