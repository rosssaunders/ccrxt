use serde::{Deserialize, Serialize};
use crate::bybit::{enums::*, EndpointType, RestResult};
use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
pub struct GetLongShortRatioRequest {
    pub category: Category,
    pub symbol: String,
    pub period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LongShortRatioInfo {
    pub symbol: String,
    #[serde(rename = "buyRatio")]
    pub buy_ratio: String,
    #[serde(rename = "sellRatio")]
    pub sell_ratio: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetLongShortRatioData {
    pub category: Category,
    pub list: Vec<LongShortRatioInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetLongShortRatioResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetLongShortRatioData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_long_short_ratio(&self, request: GetLongShortRatioRequest) -> RestResult<GetLongShortRatioResponse> {
        self.send_public_request("/v5/market/account-ratio", Some(&request), EndpointType::Market).await
    }
}

