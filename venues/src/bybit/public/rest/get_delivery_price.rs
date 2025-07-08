use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

const DELIVERY_PRICE_ENDPOINT: &str = "/v5/market/delivery-price";

#[derive(Debug, Clone, Serialize)]
pub struct GetDeliveryPriceRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeliveryPriceInfo {
    pub symbol: String,
    #[serde(rename = "deliveryPrice")]
    pub delivery_price: String,
    #[serde(rename = "deliveryTime")]
    pub delivery_time: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetDeliveryPriceData {
    pub category: Category,
    pub list: Vec<DeliveryPriceInfo>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetDeliveryPriceResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetDeliveryPriceData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_delivery_price(
        &self,
        request: GetDeliveryPriceRequest,
    ) -> RestResult<GetDeliveryPriceResponse> {
        self.send_public_request(
            DELIVERY_PRICE_ENDPOINT,
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}
