use serde::{Deserialize, Serialize};
use crate::bybit::{enums::*, EndpointType, RestResult};
use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
pub struct GetRiskLimitRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RiskLimitInfo {
    pub id: i32,
    pub symbol: String,
    #[serde(rename = "riskLimitValue")]
    pub risk_limit_value: String,
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: String,
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    #[serde(rename = "isLowestRisk")]
    pub is_lowest_risk: i32,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRiskLimitData {
    pub category: Category,
    pub list: Vec<RiskLimitInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRiskLimitResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetRiskLimitData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_risk_limit(&self, request: GetRiskLimitRequest) -> RestResult<GetRiskLimitResponse> {
        self.send_public_request("/v5/market/risk-limit", Some(&request), EndpointType::Market).await
    }
}

