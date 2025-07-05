use serde::{Deserialize, Serialize};
use crate::bybit::{enums::*, EndpointType, RestResult};
use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
pub struct GetOpenInterestRequest {
    pub category: Category,
    pub symbol: String,
    #[serde(rename = "intervalTime")]
    pub interval_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenInterestInfo {
    #[serde(rename = "openInterest")]
    pub open_interest: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetOpenInterestData {
    pub category: Category,
    pub symbol: String,
    pub list: Vec<OpenInterestInfo>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetOpenInterestResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetOpenInterestData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_open_interest(&self, request: GetOpenInterestRequest) -> RestResult<GetOpenInterestResponse> {
        self.send_public_request("/v5/market/open-interest", Some(&request), EndpointType::Market).await
    }
}

