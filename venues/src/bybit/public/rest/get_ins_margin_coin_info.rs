use serde::{Deserialize, Serialize};
use crate::bybit::{EndpointType, RestResult};
use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
pub struct GetInsMarginCoinInfoRequest {
    #[serde(rename = "productId")]
    pub product_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InsMarginCoinInfo {
    #[serde(rename = "marginCoin")]
    pub margin_coin: String,
    #[serde(rename = "conversionRate")]
    pub conversion_rate: String,
    #[serde(rename = "liquidationOrder")]
    pub liquidation_order: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsMarginCoinInfoData {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "parentUid")]
    pub parent_uid: String,
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "marginCoinInfoList")]
    pub margin_coin_info_list: Vec<InsMarginCoinInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsMarginCoinInfoResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetInsMarginCoinInfoData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_ins_margin_coin_info(&self, request: GetInsMarginCoinInfoRequest) -> RestResult<GetInsMarginCoinInfoResponse> {
        self.send_public_request("/v5/ins-loan/ensure-tokens-convert", Some(&request), EndpointType::Market).await
    }
}

impl GetInsMarginCoinInfoRequest {
    pub fn new(product_id: String) -> Self {
        Self { product_id }
    }
}