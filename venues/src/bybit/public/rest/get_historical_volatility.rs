use serde::{Deserialize, Serialize};
use crate::bybit::{enums::*, EndpointType, RestResult};
use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
pub struct GetHistoricalVolatilityRequest {
    pub category: Category,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VolatilityInfo {
    pub period: i32,
    pub value: String,
    pub time: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetHistoricalVolatilityData {
    pub category: Category,
    pub list: Vec<VolatilityInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetHistoricalVolatilityResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetHistoricalVolatilityData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_historical_volatility(&self, request: GetHistoricalVolatilityRequest) -> RestResult<GetHistoricalVolatilityResponse> {
        self.send_public_request("/v5/market/historical-volatility", Some(&request), EndpointType::Market).await
    }
}

