use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

const INSURANCE_ENDPOINT: &str = "/v5/market/insurance";

#[derive(Debug, Clone, Serialize, Default)]
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
    pub async fn get_insurance(
        &self,
        request: Option<GetInsuranceRequest>,
    ) -> RestResult<GetInsuranceResponse> {
        self.send_public_request(INSURANCE_ENDPOINT, request.as_ref(), EndpointType::Market)
            .await
    }
}
