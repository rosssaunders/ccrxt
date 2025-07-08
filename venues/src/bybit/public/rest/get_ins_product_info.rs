use serde::{Deserialize, Serialize};
use crate::bybit::{EndpointType, RestResult};
use super::client::RestClient;

const INS_PRODUCT_INFO_ENDPOINT: &str = "/v5/ins-loan/product-infos";

#[derive(Debug, Clone, Serialize)]
pub struct GetInsProductInfoRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct InsProductInfo {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "loanAmount")]
    pub loan_amount: String,
    #[serde(rename = "borrowRate")]
    pub borrow_rate: String,
    #[serde(rename = "loanPeriod")]
    pub loan_period: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsProductInfoData {
    pub list: Vec<InsProductInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsProductInfoResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetInsProductInfoData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_ins_product_info(&self) -> RestResult<GetInsProductInfoResponse> {
        self.send_public_request(INS_PRODUCT_INFO_ENDPOINT, None::<&GetInsProductInfoRequest>, EndpointType::Market).await
    }
}