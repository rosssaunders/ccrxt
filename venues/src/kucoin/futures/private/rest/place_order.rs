use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderType, ResponseHeaders, RestResponse, Result, StopType, TimeInForce,
};

const PLACE_ORDER_ENDPOINT: &str = "/api/v1/orders";

/// Place order request for futures
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    pub side: OrderSide,

    pub symbol: String,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_order: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_hold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_size: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_id: String,
}

impl super::RestClient {
    /// Place a new order
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/orders/add-order)
    pub async fn place_order(
        &self,
        request: PlaceOrderRequest,
    ) -> Result<(RestResponse<PlaceOrderResponse>, ResponseHeaders)> {
        self.post_with_request(PLACE_ORDER_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kucoin::spot::{OrderSide, OrderType, TimeInForce};

    #[test]
    fn test_place_order_request_serialization() {
        let request = PlaceOrderRequest {
            client_oid: Some("test123".to_string()),
            side: OrderSide::Buy,
            symbol: "XBTUSDTM".to_string(),
            order_type: OrderType::Limit,
            leverage: Some("10".to_string()),
            reduce_only: Some(false),
            close_order: Some(false),
            force_hold: Some(false),
            size: Some(1),
            price: Some("50000".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            post_only: Some(false),
            hidden: Some(false),
            iceberg: Some(false),
            visible_size: None,
            remark: Some("test order".to_string()),
            stop: None,
            stop_price_type: None,
            stop_price: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(json.contains("buy"));
        assert!(json.contains("limit"));
    }
}
