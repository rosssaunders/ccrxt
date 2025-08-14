use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

const RISK_LIMIT_ENDPOINT: &str = "/v5/market/risk-limit";

/// Request parameters for getting risk limit
#[derive(Debug, Clone, Serialize)]
pub struct GetRiskLimitRequest {
    /// Product type (Linear or Inverse)
    pub category: Category,

    /// Symbol name. Required for Linear, optional for Inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Risk limit information
#[derive(Debug, Clone, Deserialize)]
pub struct RiskLimitInfo {
    /// Risk limit ID
    pub id: i32,

    /// Symbol name
    pub symbol: String,

    /// Position limit value corresponding to risk ID
    #[serde(rename = "riskLimitValue")]
    pub risk_limit_value: String,

    /// Maintenance margin rate
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: String,

    /// Initial margin rate
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,

    /// Whether this is the lowest risk ID (1: true, 0: false)
    #[serde(rename = "isLowestRisk")]
    pub is_lowest_risk: i32,

    /// Maximum leverage
    #[serde(rename = "maxLeverage")]
    pub max_leverage: String,
}

/// Risk limit data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetRiskLimitData {
    /// Product type
    pub category: Category,

    /// Array of risk limit info
    pub list: Vec<RiskLimitInfo>,
}

/// Response from the risk limit endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetRiskLimitResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetRiskLimitData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get risk limit
    ///
    /// Query for the risk limit table for Linear/Inverse perpetual and futures contracts.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/risk-limit)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The risk limit request parameters including:
    ///   - `category`: Product type (Linear or Inverse)
    ///   - `symbol`: Optional symbol name (required for Linear)
    ///
    /// # Returns
    /// A result containing the risk limit response with risk limit tiers or an error
    pub async fn get_risk_limit(
        &self,
        request: GetRiskLimitRequest,
    ) -> RestResult<GetRiskLimitResponse> {
        self.send_public_request(RISK_LIMIT_ENDPOINT, Some(&request), EndpointType::Market)
            .await
    }
}
