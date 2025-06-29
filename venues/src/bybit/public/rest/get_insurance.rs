use serde::{Deserialize, Serialize};
use crate::bybit::{EndpointType, RestResult};
use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
pub struct GetInsuranceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InsuranceInfo {
    pub coin: String,
    pub balance: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsuranceData {
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
    pub list: Vec<InsuranceInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsuranceResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetInsuranceData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_insurance(&self, request: Option<GetInsuranceRequest>) -> RestResult<GetInsuranceResponse> {
        self.send_public_request("/v5/market/insurance", request.as_ref(), EndpointType::Market).await
    }
}

impl GetInsuranceRequest {
    pub fn new() -> Self { Self { coin: None } }
    pub fn coin(mut self, coin: String) -> Self { self.coin = Some(coin); self }
}